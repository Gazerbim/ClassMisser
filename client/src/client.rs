use std::{
    io::{Read, Write},
    net::TcpStream,
};

use serde_json::Value;

use crate::types::protocol::Response;

pub struct Client {
    stream: TcpStream,
    receive_buffer: Vec<u8>,
}

impl Client {
    pub fn new(host: &String) -> Client {
        let stream = TcpStream::connect(host).unwrap();

        stream
            .set_nonblocking(true)
            .expect("set_nonblocking call failed");

        Client {
            stream,
            receive_buffer: Vec::new(),
        }
    }

    pub fn send(&mut self, message: &String) -> bool {
        match self.stream.write_all(message.as_bytes()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn receive_message(&mut self) -> Option<String> {
        // verifier si un message complet est deja dans le buffer
        if let Some(pos) = self.receive_buffer.iter().position(|&b| b == b'$') {
            let mut line: Vec<u8> = self.receive_buffer.drain(..=pos).collect();

            // Retirer le '\n'
            line.pop();

            // Retirer le '\r' éventuel
            if line.last() == Some(&b'\r') {
                line.pop();
            }

            return Some(String::from_utf8_lossy(&line).into_owned());
        }

        // lire les données du socket jusqu'à obtenir un message complet
        let mut temp = [0u8; 4096];

        loop {
            match self.stream.read(&mut temp) {
                Ok(0) => return None,

                Ok(size) => {
                    self.receive_buffer.extend_from_slice(&temp[..size]);

                    // Dès qu'un message complet est disponible, on le retourne
                    if let Some(pos) = self.receive_buffer.iter().position(|&b| b == b'$') {
                        let mut line: Vec<u8> = self.receive_buffer.drain(..=pos).collect();

                        line.pop();

                        if line.last() == Some(&b'\r') {
                            line.pop();
                        }

                        return Some(String::from_utf8_lossy(&line).into_owned());
                    }
                }

                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {
                    continue;
                }

                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    return None;
                }

                Err(_) => return None,
            }
        }
    }

    pub fn parse_response(&self, message: &str) -> Response {
        //à améliorer, en POC pour l'instant
        //println!("message reçu: {}", message);
        let request: Response = match serde_json::from_str(message) {
            Ok(response) => response,
            Err(_e) => {
                eprint!("message serveur non reconnu: {}", _e);
                Response {
                    request_type: String::from(""),
                    header: String::from(""),
                    data: Value::Bool(false),
                }
            }
        };
        request
    }
}
