use console_engine::Color;
use console_engine::KeyCode;
use console_engine::pixel;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn start_client() {
    //initialiser tcp
    let mut tcp_stream = TcpStream::connect("127.0.0.1:9001").unwrap();
    tcp_stream
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    let mut engine = console_engine::ConsoleEngine::init_fill(20).unwrap();
    let mut labyrinth = [[false; 30]; 100];
    let mut writed_in_buffer = String::with_capacity(98);
    let mut to_display: [String; 26] = Default::default();
    let mut displayed: i32 = 0;
    for x in 0..100 {
        labyrinth[x][0] = true;
        labyrinth[x][29] = true;
        labyrinth[x][27] = true;
        if x == 0 || x == 99 {
            for y in 0..30 {
                labyrinth[x][y] = true;
            }
        }
    }

    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.clear_screen(); // reset the screen

        add_to_input(&engine, &mut writed_in_buffer);
        if engine.is_key_pressed(console_engine::KeyCode::Esc) {
            // if the user presses 'q' :
            break; // exits app
        }
        if engine.is_key_pressed(console_engine::KeyCode::Enter) {
            tcp_stream.write_all(writed_in_buffer.as_bytes()).unwrap();
            writed_in_buffer = String::with_capacity(98);
        }

        let mut buffer = [0; 98];
        match tcp_stream.read(&mut buffer) {
            Ok(_size) => {
                let message = String::from_utf8_lossy(&buffer[.._size]);
                if message != "" {
                    to_display[displayed as usize] = String::from(message);
                    displayed = displayed + 1;
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
            }
            Err(e) => panic!("encountered IO error: {e}"),
        };

        display(&mut engine, &labyrinth);
        if !to_display.is_empty() {
            for i in 0i32..displayed {
                engine.print(1, 1 + i, &to_display[i as usize]);
            }
        }
        engine.print(1, 28, &writed_in_buffer);

        engine.draw(); // draw the screen
    }
}

fn add_to_input(e: &console_engine::ConsoleEngine, h: &mut String) {
    let chars = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ' ', '!', '?', ',', '.',
    ];
    for c in chars {
        if e.is_key_pressed(KeyCode::Char(c)) {
            h.push(c);
        }
    }
    if e.is_key_pressed(console_engine::KeyCode::Backspace) {
        h.pop();
    }
}

fn display(e: &mut console_engine::ConsoleEngine, l: &[[bool; 30]; 100]) {
    for x in 0i32..100 {
        for y in 0i32..30 {
            if l[x as usize][y as usize] {
                e.set_pxl(x, y, pixel::pxl_fbg('#', Color::Red, Color::DarkBlue));
            }
        }
    }
}
