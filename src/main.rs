// Uncomment this block to pass the first stage
use std::{net::TcpListener, io::{Write, Read}};
use std::thread;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

     //Uncomment this block to pass the first stage
    
     let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
     for stream in listener.incoming() {
        thread::spawn(|| {
            match stream {
                Ok(mut stream) => {
                    println!("accepted new connection");
                    loop {
                       let mut buf: [u8; 1000] = [0; 1000];
                       stream.read(&mut buf).expect("Couldn't read stream to buffer!");
                       println!("{:?}", buf);
                       stream.write(b"+PONG\r\n").expect("Couldn't respond to PING!");
                    }
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        });
     }
}
