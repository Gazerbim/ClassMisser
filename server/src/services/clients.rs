use mio::net::{TcpStream};
use mio::{Token};
use std::collections::HashMap;

pub struct Client{
    pub socket: TcpStream,
    pub buffer: Vec<u8>,
    pub outgoing: Vec<u8>,
    pub sent: usize,
    pub token: Token,
}

impl Client{
    pub fn new(socket: TcpStream, token:Token)->Client{
        Client { socket:socket, buffer:Vec::new(), outgoing:Vec::new(), sent:0, token:token}
    }
}


pub struct Clients{
    pub list: HashMap<Token, Client>
}

impl Clients{
    pub fn new()->Clients{
        Clients { list: HashMap::new() }
    }

    pub fn insert(&mut self, token:Token, client:Client){
        self.list.insert(token, client);
    }
}