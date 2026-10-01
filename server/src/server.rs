use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;
use std::time;

use crate::services::lobbies;

pub struct Server {
    stream_list: Vec<TcpStream>,
    rx: std::sync::mpsc::Receiver<TcpStream>,
}

impl Server {
    pub fn start(rx: std::sync::mpsc::Receiver<TcpStream>) -> Server {
        Server {
            stream_list: vec![],
            rx: rx,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.get_new_connection();
            thread::sleep(time::Duration::from_millis(1));

            //traitement ici
            let mut closed_tcps: Vec<i32> = Vec::new();
            for i in 0..self.stream_list.len() {
                let mut message = self.receive(i as i32, &mut closed_tcps);
                if message != "" {
                    message.push_str(&" from server");
                    self.send(i, message);
                }
            }
            //retire les sockets fermés
            for i in (0..closed_tcps.len()).rev() {
                self.stream_list.remove(closed_tcps[i] as usize);
            }
            thread::sleep(time::Duration::from_millis(1));
        }
    }

    pub fn get_new_connection(&mut self) {
        match self.rx.try_recv() {
            Ok(_new_stream) => {
                _new_stream
                    .set_nonblocking(true)
                    .expect("set_nonblocking call failed");
                // _new_stream
                //     .set_read_timeout(Some(Duration::from_millis(250)))
                //     .expect("set_read_timeout call failed");
                self.stream_list.push(_new_stream);
                for i in 0..self.stream_list.len() {
                    println!(
                        "User {} connected",
                        self.stream_list[i].peer_addr().unwrap()
                    );
                }
            }
            Err(e) => {
                if e != mpsc::TryRecvError::Empty {
                    println!("couldn't get new stream: {e:?}");
                }
            }
        }
    }

    fn receive(&mut self, index: i32, closed_tcps: &mut Vec<i32>) -> String {
        let mut buffer = [0; 1024];
        match self.stream_list[index as usize].read(&mut buffer) {
            Ok(_size) => {
                let message = String::from_utf8_lossy(&buffer[.._size]);
                if message != "" {
                    println!(
                        "Message: {}, de {}",
                        message,
                        self.stream_list[index as usize].peer_addr().unwrap()
                    );
                } else {
                    closed_tcps.push(index as i32);
                }
                return String::from(message);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
            }
            Err(e) => panic!("encountered IO error: {e}"),
        };
        return "".to_string();
    }

    fn send(&mut self, index: usize, message: String) {
        self.stream_list[index]
            .write_all(message.as_bytes())
            .unwrap();
    }
}
