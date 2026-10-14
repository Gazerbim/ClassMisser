use mio::net::TcpListener;
use mio::{Events, Interest, Poll, Token};
use serde_json::Value;
use std::io::{self, Read, Write};

use std::net::SocketAddr;

use crate::services::clients::{Client, Clients};
use crate::services::lobbies::Lobbies;
use crate::services::players::{Player, Players};
use crate::services::protocol::Request;

const SERVER: Token = Token(0);

pub struct Server {
    address: String,
    port: String,
    lobbies: Lobbies,
    clients: Clients,
    poll: Poll,
    players: Players,
}

impl Server {
    pub fn new(port: u16) -> Server {
        Server {
            address: String::from("127.0.0.1"),
            port: port.to_string(),
            lobbies: Lobbies::new(),
            clients: Clients::new(),
            poll: Poll::new().unwrap(),
            players: Players::new(),
        }
    }

    // MAIN LOOP

    pub fn start_server(&mut self) -> io::Result<()> {
        let mut events = Events::with_capacity(128);

        let address: SocketAddr = format!("{}:{}", self.address, self.port).parse().unwrap();

        let mut listener = TcpListener::bind(address)?;

        self.poll
            .registry()
            .register(&mut listener, SERVER, Interest::READABLE)?;

        let mut next_token = 1;

        println!("Serveur démarré sur {}:{}", self.address, self.port);

        loop {
            self.poll.poll(&mut events, None)?;

            for event in events.iter() {
                let token = event.token();
                let readable = event.is_readable();
                let writable = event.is_writable();

                // Nouvelle connexion
                if token == SERVER {
                    self.accept_connections(&mut listener, &mut next_token)?;
                    continue;
                }

                // Événement client
                self.handle_client_event(token, readable, writable)?;
            }
        }
    }

    // CONNECTION MANAGEMENT

    fn accept_connections(
        &mut self,
        listener: &mut TcpListener,
        next_token: &mut usize,
    ) -> io::Result<()> {
        loop {
            match listener.accept() {
                Ok((mut socket, addr)) => {
                    let token = Token(*next_token);
                    *next_token += 1;

                    println!("Connexion : {}", addr);

                    self.poll
                        .registry()
                        .register(&mut socket, token, Interest::READABLE)?;

                    self.clients.insert(token, Client::new(socket, token));
                }

                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    break;
                }

                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {
                    continue;
                }

                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    fn disconnect_client(&mut self, token: Token) {
        if let Some(mut client) = self.clients.list.remove(&token) {
            let _ = self.poll.registry().deregister(&mut client.socket);
            self.lobbies.player_disconnection(&token);
            println!("Client {:?} déconnecté", token);
        }
    }

    // CLIENT EVENT MANAGEMENT

    fn handle_client_event(
        &mut self,
        token: Token,
        readable: bool,
        writable: bool,
    ) -> io::Result<()> {
        // Réception des messages
        let mut disconnected = false;

        if readable {
            let (messages, disconnected_read) = self.read_messages(token);
            disconnected = disconnected_read;

            // Traitement des messages reçus
            if !disconnected {
                for message in messages {
                    println!("Client {:?} : {}", token, message);

                    self.handle_message(token, &message)?;
                }
            }
        }

        if disconnected {
            self.disconnect_client(token);
            return Ok(());
        }

        // Envoi des messages en attente
        if writable {
            disconnected = self.flush_client(token);
        }

        if disconnected {
            self.disconnect_client(token);
            return Ok(());
        }

        // Mise à jour des événements Mio
        self.update_client_interest(token)?;

        Ok(())
    }

    // READING messages

    fn read_messages(&mut self, token: Token) -> (Vec<String>, bool) {
        let mut messages = Vec::new();
        let mut disconnected = false;

        for e in self.clients.list.keys() {
            self.clients.list.get(&e).unwrap();
        }
        if let Some(client) = self.clients.list.get_mut(&token) {
            let mut temp = [0u8; 4096];

            loop {
                match client.socket.read(&mut temp) {
                    Ok(0) => {
                        disconnected = true;
                        break;
                    }

                    Ok(size) => {
                        client.buffer.extend_from_slice(&temp[..size]);

                        while let Some(pos) = client.buffer.iter().position(|&b| b == b'\n') {
                            let line: Vec<u8> = client.buffer.drain(..=pos).collect();

                            let message = String::from_utf8_lossy(&line[..line.len() - 1])
                                .trim_end_matches('\r')
                                .to_string();

                            println!("message = {}", &message);
                            messages.push(message);
                        }
                    }

                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        break;
                    }

                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {
                        continue;
                    }

                    Err(_) => {
                        disconnected = true;
                        break;
                    }
                }
            }
        }

        (messages, disconnected)
    }

    // =========================================================
    // WRITING
    // =========================================================

    fn flush_client(&mut self, token: Token) -> bool {
        let Some(client) = self.clients.list.get_mut(&token) else {
            return false;
        };

        loop {
            if client.sent == client.outgoing.len() {
                client.outgoing.clear();
                client.sent = 0;

                break;
            }

            match client.socket.write(&client.outgoing[client.sent..]) {
                Ok(0) => {
                    return true;
                }

                Ok(size) => {
                    client.sent += size;
                }

                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    break;
                }

                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {
                    continue;
                }

                Err(_) => {
                    return true;
                }
            }
        }

        false
    }

    fn update_client_interest(&mut self, token: Token) -> io::Result<()> {
        // Mise à jour des événements Mio
        if let Some(client) = self.clients.list.get_mut(&token) {
            let interest = if client.sent < client.outgoing.len() {
                Interest::READABLE.add(Interest::WRITABLE)
            } else {
                Interest::READABLE
            };

            self.poll
                .registry()
                .reregister(&mut client.socket, token, interest)?;
        }

        Ok(())
    }

    // MESSAGE SENDING

    // Envoi à un seul client
    pub fn send_to_client(&mut self, token: Token, message: &str) -> io::Result<()> {
        if let Some(client) = self.clients.list.get_mut(&token) {
            client.outgoing.extend_from_slice(message.as_bytes());

            self.update_client_interest(token)?;
        }

        Ok(())
    }

    // Envoi à une liste de clients
    pub fn send_to_clients(&mut self, tokens: &[Token], message: &str) -> io::Result<()> {
        for &token in tokens {
            self.send_to_client(token, message)?;
        }

        Ok(())
    }

    // Broadcast à un lobby
    pub fn broadcast_to_lobby(&mut self, lobby: usize, message: &str) -> io::Result<()> {
        let tokens = self
            .lobbies
            .list
            .get(&lobby)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Lobby introuvable"))?
            .list_players
            .clone();

        self.send_to_clients(&tokens, message)?;

        Ok(())
    }

    fn handle_message(&mut self, sender: Token, message: &str) -> io::Result<()> {
        // Décodage JSON
        let parsed: Value = match serde_json::from_str(message) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("JSON invalide : {}", e);

                return self.send_to_client(
                    sender,
                    &Self::make_response(
                        "error",
                        "error",
                        serde_json::json!({"message": "Invalid JSON"}),
                    ),
                );
            }
        };

        //######################
        let request_value = parsed.get("request").unwrap_or(&parsed);

        let request: Request = match serde_json::from_value(request_value.clone()) {
            Ok(request) => request,
            Err(e) => {
                eprintln!("Requête invalide : {}", e);

                return self.send_to_client(
                    sender,
                    &Self::make_response(
                        "error",
                        "error",
                        serde_json::json!({"message": "Invalid request"}),
                    ),
                );
            }
        };

        println!(
            "Requête reçue : type={}, header={}",
            request.request_type, request.header
        );

        match (request.request_type.as_str(), request.header.as_str()) {
            ("username", "give_new_username") => {
                self.handle_give_new_username(sender, &request.data)?;
            }

            ("lobbies", _) => {
                let mut receiver = sender.clone();
                let response = self.lobbies.handle_request(
                    sender,
                    request.header.as_str(),
                    &request.data,
                    &mut receiver,
                );
                if response != String::from("") {
                    self.send_to_client(receiver, &response)?;
                }
            }
            ("game", "start_game") => {
                self.handle_start_game(sender, &request.data)?;
            }

            ("game", _) => {
                let lobby_name = request.data["lobby_name"].as_str().unwrap().to_string();
                match self.lobbies.get_lobby_id_by_name(lobby_name) {
                    Some(id) => {
                        let l = self.lobbies.list.get_mut(&id).unwrap();
                        let response =
                            l.handle_request(sender, request.header.as_str(), &request.data);
                        if response != "".to_string() {
                            //à priori, dans la pertie, tous les messages sont en broadcast je dirais
                            self.broadcast_to_lobby(id, response.as_str()).unwrap();
                        }
                    }
                    None => {}
                }
            }

            _ => {
                eprintln!(
                    "Requête inconnue : {} / {}",
                    request.request_type, request.header
                );

                // self.send_to_client(
                //     sender,
                //     &Self::make_response(
                //         "error",
                //         serde_json::json!({"message": "Unknown request"}),
                //     ),
                // )?;
            }
        }

        Ok(())
    }

    // pour construire les réponses JSON
    fn make_response(response_type: &str, response_header: &str, data: Value) -> String {
        serde_json::json!({
            "type": response_type,
            "header": response_header,
            "data": data
        })
        .to_string()
            + "\n"
    }

    // USERNAME

    fn handle_give_new_username(&mut self, sender: Token, data: &Value) -> io::Result<()> {
        let username = data.get("username").and_then(Value::as_str).unwrap_or("");

        println!("Nouveau pseudo demandé : {}", username);

        let accepted = self
            .players
            .add_player(sender, Player::new(String::from(username)));

        self.send_to_client(
            sender,
            &Self::make_response(
                "username",
                "is_username_available",
                serde_json::json!({
                    "accept": accepted
                }),
            ),
        )
    }

    // GAME

    fn handle_start_game(&mut self, sender: Token, data: &Value) -> io::Result<()> {
        // let player = self.players.get_player(sender).unwrap();
        // let lobby: usize = player.lobby.unwrap();
        let n = data["players"].as_i64().unwrap();
        let mut id = -1;
        if n > 1 && n < 5 {
            id = self.lobbies.get_player_lobby_id(sender);
        }
        if id != -1 {
            let id = id as usize;

            let lobby = self.lobbies.list.get_mut(&id).unwrap();
            lobby.in_game = true;
            lobby.number_of_players = n as usize;

            self.broadcast_to_lobby(
                id,
                &Self::make_response(
                    "game",
                    "game_is_starting",
                    serde_json::json!({
                        "accept": true,
                        "color": "white"
                    }),
                ),
            )
        } else {
            self.send_to_client(
                sender,
                &Self::make_response(
                    "game",
                    "game_is_starting",
                    serde_json::json!({
                        "accept": false
                    }),
                ),
            )
        }
    }

    fn handle_is_the_game_starting(&mut self, sender: Token) -> io::Result<()> {
        // vérifier l'état de la partie

        self.send_to_client(
            sender,
            &Self::make_response(
                "game",
                "game_is_starting",
                serde_json::json!({
                    "accept": false,
                    "color": ""
                }),
            ),
        )
    }

    fn handle_quitted_game(&mut self, sender: Token) -> io::Result<()> {
        // gerer le départ du joueur

        self.send_to_client(
            sender,
            &Self::make_response("game", "game_quitted", serde_json::json!({})),
        )
    }
}
