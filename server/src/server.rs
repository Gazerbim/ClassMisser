use mio::net::TcpListener;
use mio::{Events, Interest, Poll, Token};
use serde_json::Value;
use std::io::{self, Read, Write};

use std::net::SocketAddr;

use crate::services::clients::{Client, Clients};
use crate::services::lobbies::Lobbies;
use crate::services::protocol::Request;

const SERVER: Token = Token(0);

pub struct Server {
    address: String,
    port: String,
    lobbies: Lobbies,
    clients: Clients,
    poll: Poll,
}

impl Server {
    pub fn new(port: u16) -> Server {
        Server {
            address: String::from("127.0.0.1"),
            port: port.to_string(),
            lobbies: Lobbies::new(),
            clients: Clients::new(),
            poll: Poll::new().unwrap(),
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
            .get(lobby)
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
                    &Self::make_response("error", serde_json::json!({"message": "Invalid JSON"})),
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
                //self.handle_list_available_lobbies(sender)?;
                let response =
                    self.lobbies
                        .handle_request(sender, request.header.as_str(), &request.data);
                self.send_to_client(sender, &response)?;
            }
            //             ("lobbies", "enter_lobby") => {
            //                 self.handle_enter_lobby(sender, &request.data)?;
            //             }
            //
            //             ("lobbies", "quit_lobby") => {
            //                 self.handle_quit_lobby(sender)?;
            //             }
            //
            //             ("lobbies", "lobby_toujours_actif") => {
            //                 self.handle_lobby_toujours_actif(sender)?;
            //             }
            ("game", "start_game") => {
                self.handle_start_game(sender, &request.data)?;
            }

            ("game", "is_the_game_starting") => {
                self.handle_is_the_game_starting(sender)?;
            }

            ("game", "quitted_game") => {
                self.handle_quitted_game(sender)?;
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
    fn make_response(response_type: &str, data: Value) -> String {
        serde_json::json!({
            "type": response_type,
            "data": data
        })
        .to_string()
            + "\n"
    }

    // USERNAME

    fn handle_give_new_username(&mut self, sender: Token, data: &Value) -> io::Result<()> {
        let username = data.get("username").and_then(Value::as_str).unwrap_or("");

        println!("Nouveau pseudo demandé : {}", username);

        self.send_to_client(
            sender,
            &Self::make_response(
                "is_username_available",
                serde_json::json!({
                    "accept": true
                }),
            ),
        )
    }

    // LOBBIES

    fn handle_create_lobby(&mut self, sender: Token, data: &Value) -> io::Result<()> {
        let name = data.get("name").and_then(Value::as_str).unwrap_or("");

        println!("Création du lobby : {}", name);

        self.send_to_client(
            sender,
            &Self::make_response(
                "created_lobby",
                serde_json::json!({
                    "accept": true
                }),
            ),
        )
    }

    fn handle_enter_lobby(&mut self, sender: Token, _data: &Value) -> io::Result<()> {
        // rejoindre le lobby demandé

        self.send_to_client(
            sender,
            &Self::make_response(
                "enter_lobby",
                serde_json::json!({
                    "accept": true
                }),
            ),
        )
    }

    fn handle_quit_lobby(&mut self, sender: Token) -> io::Result<()> {
        // retirer le joueur de son lobby

        self.send_to_client(
            sender,
            &Self::make_response("quit_lobby", serde_json::json!({})),
        )
    }

    fn handle_lobby_toujours_actif(&mut self, sender: Token) -> io::Result<()> {
        // verifier si le lobby existe toujours

        self.send_to_client(
            sender,
            &Self::make_response(
                "list_available_lobbies",
                serde_json::json!({
                    "accept": true
                }),
            ),
        )
    }

    // GAME

    fn handle_start_game(&mut self, sender: Token, data: &Value) -> io::Result<()> {
        let number_of_player = data
            .get("number_of_player")
            .and_then(Value::as_u64)
            .unwrap_or(0);

        println!(
            "Demande de lancement de partie avec {} joueurs",
            number_of_player
        );

        self.send_to_client(
            sender,
            &Self::make_response(
                "game_is_starting",
                serde_json::json!({
                    "accept": true,
                    "color": "white"
                }),
            ),
        )
    }

    fn handle_is_the_game_starting(&mut self, sender: Token) -> io::Result<()> {
        // vérifier l'état de la partie

        self.send_to_client(
            sender,
            &Self::make_response(
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
            &Self::make_response("game_quitted", serde_json::json!({})),
        )
    }
}
