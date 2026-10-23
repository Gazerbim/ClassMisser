use mio::Token;
use mio::net::TcpStream;
use std::collections::HashMap;

pub struct Client {
    pub socket: TcpStream,
    pub buffer: Vec<u8>,
    pub outgoing: Vec<u8>,
    pub sent: usize,
    pub token: Token,
    pub connection_state: bool,
}

impl Client {
    pub fn new(socket: TcpStream, token: Token) -> Client {
        Client {
            socket: socket,
            buffer: Vec::new(),
            outgoing: Vec::new(),
            sent: 0,
            token: token,
            connection_state: false,
        }
    }
}

pub struct Clients {
    pub list: HashMap<Token, Client>,
}

impl Clients {
    pub fn new() -> Clients {
        Clients {
            list: HashMap::new(),
        }
    }

    pub fn insert(&mut self, token: Token, client: Client) {
        self.list.insert(token, client);
    }

    pub fn get_connection_value(&self, token: &Token) -> bool {
        let c = self.list.get(token).unwrap();
        c.connection_state
    }

    pub fn set_client_connection_state(&mut self, token: &Token, state: bool) {
        let c = self.list.get_mut(token).unwrap();
        c.connection_state = state;
    }
}
