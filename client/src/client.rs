use std::{io::Read, io::Write, net::TcpStream};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(host: &String) -> Client {
        let stream = TcpStream::connect(host).unwrap();
        stream
            .set_nonblocking(true)
            .expect("set_nonblocking call failed");
        Client { stream: stream }
    }

    pub fn send(&mut self, message: &String) -> bool {
        match self.stream.write_all(message.as_bytes()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn receive(&mut self, message: &mut String) -> bool {
        let mut buffer = [0; 98];
        match self.stream.read(&mut buffer) {
            Ok(_size) => {
                let new_message = String::from_utf8_lossy(&buffer[.._size]).into_owned();
                *message = new_message;
                true
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                true
            }
            Err(_e) => false, //panic!("encountered IO error: {e}"),
        }
    }

    pub fn request(&mut self, req: &String) -> String {
        self.send(req);
        self.stream
            .set_nonblocking(false)
            .expect("set_nonblocking call failed");
        let mut response: String = String::from("");
        let worked = self.receive(&mut response);
        self.stream
            .set_nonblocking(true)
            .expect("set_nonblocking call failed");
        if worked {
            return response;
        } else {
            return String::from("");
        }
    }
}
