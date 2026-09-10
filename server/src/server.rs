use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time;

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:9001").unwrap();
    let mut stream_list: Vec<TcpStream> = Vec::new();

    let (tx, rx) = mpsc::channel();

    //thread qui cherche de nouvelles connections
    thread::spawn(move || {
        look_for_connections(&listener, &tx);
    });

    //thread principal du serveur
    loop {
        get_new_connection(&rx, &mut stream_list);
        thread::sleep(time::Duration::from_millis(1));

        //traitement ici
        let mut closed_tcps: Vec<i32> = Vec::new();
        for i in 0..stream_list.len() {
            let mut buffer = [0; 1024];
            //let size = stream_list[i].read(&mut buffer).unwrap();
            match stream_list[i].read(&mut buffer) {
                Ok(_size) => {
                    let message = String::from_utf8_lossy(&buffer[.._size]);
                    if message != "" {
                        println!("Message: {}, de {}", message, i);
                        //broadcast
                        broadcast(&mut stream_list, String::from(message));
                    } else {
                        closed_tcps.push(i as i32);
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP
                }
                Err(e) => panic!("encountered IO error: {e}"),
            };
        }
        //retire les sockets fermés
        for i in (0..closed_tcps.len()).rev() {
            stream_list.remove(closed_tcps[i] as usize);
        }
    }
}

fn look_for_connections(listener: &TcpListener, tx: &std::sync::mpsc::Sender<TcpStream>) {
    loop {
        match listener.accept() {
            Ok((_socket, addr)) => {
                println!("new client: {addr:?}");
                tx.send(_socket).unwrap();
            }
            Err(e) => println!("couldn't get client: {e:?}"),
        }
        thread::sleep(time::Duration::from_millis(1));
    }
}

fn get_new_connection(rx: &std::sync::mpsc::Receiver<TcpStream>, stream_list: &mut Vec<TcpStream>) {
    match rx.try_recv() {
        Ok(_new_stream) => {
            _new_stream
                .set_nonblocking(true)
                .expect("set_nonblocking call failed");
            // _new_stream
            //     .set_read_timeout(Some(Duration::from_millis(250)))
            //     .expect("set_read_timeout call failed");
            stream_list.push(_new_stream);
            for i in 0..stream_list.len() {
                println!("User {} connected", stream_list[i].peer_addr().unwrap());
            }
        }
        Err(e) => {
            if e != mpsc::TryRecvError::Empty {
                println!("couldn't get new stream: {e:?}");
            }
        }
    }
}

fn broadcast(stream_list: &mut Vec<TcpStream>, message: String) {
    for j in 0..stream_list.len() {
        stream_list[j].write_all(message.as_bytes()).unwrap();
    }
}