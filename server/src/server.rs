use std::io::{self, Read, Write};
use mio::net::{TcpListener};
use mio::{Token, Events, Interest, Poll};

use std::net::SocketAddr;

use crate::services::lobbies::{Lobbies};
use crate::services::clients::{Clients, Client};


const SERVER: Token = Token(0);

pub struct Server{
    address: String,
    port:String,
    lobbies: Lobbies,
    clients: Clients,
    poll: Poll,
}

impl Server{

    pub fn new(port: u16) -> Server {
        Server {
            address: String::from("127.0.0.1"),
            port: port.to_string(),
            lobbies: Lobbies::new(),
            clients: Clients::new(),
            poll:Poll::new().unwrap(),
        }
    }

    pub fn start_server(&mut self)->io::Result<()>{
        let mut events = Events::with_capacity(128);

        let address:SocketAddr = format!("{}:{}", self.address, self.port).parse().unwrap();
        let mut listener = TcpListener::bind(address)?;

        self.poll.registry().register(&mut listener, SERVER, Interest::READABLE,)?;

        let mut next_token = 1;

        println!("Serveur démarré sur {}:{}", self.address, self.port);

        //main loop
        loop{
            //attendre les evts réseau
            self.poll.poll(&mut events, None)?;

            for event in events.iter(){
                let token = event.token();
                let readable = event.is_readable();
                let writable = event.is_writable();
                
                //Nouvelle connexion
                if token == SERVER{
                    loop{
                        match listener.accept(){
                            Ok((mut socket, addr))=>{
                                let token = Token(next_token);
                                next_token += 1;

                                print!("Connexion:{}", addr);

                                self.poll.registry().register(&mut socket, token, Interest::READABLE)?;
                                self.clients.insert(token, Client::new(socket, token));

                            }

                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock=>{
                                break;
                            }

                            Err(e) => {return Err(e)}
                        }
                    }
                    continue;
                }

                //message d'un client

                let mut messages = Vec::new();
                let mut disconnected = false;

                if let Some(client) = self.clients.list.get_mut(&token){
                    if readable{
                        let mut temp = [0u8; 4096];
                        loop{
                            match client.socket.read(&mut temp) {
                                Ok(0) => {
                                    disconnected = true;
                                    break;
                                }

                                Ok(size)=>{
                                    client.buffer.extend_from_slice(&temp[..size]);
                                    while let Some(pos)=client.buffer.iter().position(|&b| b==b'\n') {
                                        let line: Vec<u8> = client.buffer.drain(..=pos).collect();
                                        let message = String::from_utf8_lossy(&line[..line.len()-1]).trim_end_matches('\r').to_string();

                                        messages.push(message);
                                    }
                                }

                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock=>{
                                    break;
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::Interrupted=>{
                                    continue;
                                }
                                Err(_) =>{
                                    disconnected = true;
                                    break;
                                }
                            }
                        }
                    }
                }

                //logique du jeu

                if !disconnected{
                    for message in messages{
                        println!("Client {:?} : {}", token, message);
                        let responses = handle_message(token, &message);

                        //Ajout des réponses dans les buffers
                        for (recipient, response) in responses {
                            if let Some(client) = self.clients.list.get_mut(&recipient) {

                                client.outgoing.extend_from_slice(response.as_bytes());

                                self.poll.registry().reregister(
                                    &mut client.socket,
                                    recipient,
                                    Interest::READABLE.add(Interest::WRITABLE),
                                )?;
                            }
                        }
                    }
                }

                //envoi des réponses
                if let Some(client) = self.clients.list.get_mut(&token){
                    if writable && !disconnected{
                        loop{
                            if client.sent == client.outgoing.len(){
                                client.outgoing.clear();
                                client.sent = 0;
                                break;
                            }

                            match client.socket.write(&client.outgoing[client.sent..]){
                                Ok(0) => {
                                    disconnected = true;
                                    break;
                                }
                                Ok(size)=>{
                                    client.sent += size;
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock=>{
                                    break;
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::Interrupted=>{
                                    continue;
                                }
                                Err(_) =>{
                                    disconnected = true;
                                    break;
                                }
                            }
                        }
                    }
                }

                if disconnected{

                    if let Some(mut client) = self.clients.list.remove(&token){
                        let _ = self.poll.registry().deregister(&mut client.socket);
                    }
                    println!("Client {:?} déconnecté", token);

                }else if let Some(client) = self.clients.list.get_mut(&token){

                    let interest: Interest = if client.sent < client.outgoing.len() {
                        Interest::READABLE.add(Interest::WRITABLE)
                    }else{
                        Interest::READABLE
                    };

                    self.poll.registry().reregister(&mut client.socket, token, interest)?;

                }
            }

        }
    }
   

    fn broadcast_to_lobby(
        &mut self,
        lobby: usize,
        message: &str
    ) -> io::Result<()> {

        // On récupère les Token des joueurs du lobby.
        // clone() évite de conserver un emprunt sur self.lobbies.
        let clients_tokens = self.lobbies.list
            .get(lobby)
            .ok_or_else(|| io::Error::new(
                io::ErrorKind::NotFound,
                "Lobby introuvable"
            ))?
            .list_players.clone();

        // On parcourt les Token et on modifie directement les clients.
        for token in clients_tokens {

            if let Some(client) = self.clients.list.get_mut(&token) {

                client.outgoing.extend_from_slice(message.as_bytes());

                self.poll.registry().reregister(
                    &mut client.socket,
                    client.token,
                    Interest::READABLE.add(Interest::WRITABLE),
                )?;
            }
        }

        Ok(())
    }


}

fn handle_message(
    sender: Token,
    message: &str,
) -> Vec<(Token, String)> {

    println!("Traitement du message : {}", message);

    // Pour le moment, on répond uniquement à l'expéditeur.
    vec![
        (sender, format!("ACK: {}\n", message))
    ]
}
