use std::{net::{TcpListener , TcpStream}, io::{Read, Write}};
use bridge::{config::Config, packet::Packet};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8;4096];
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            continue;
        }
        let packet = Packet::OUT("hello traveller\n".to_string());
        if let Ok(bytes_written) = stream.write(&packet.serialize()) {
            println!("{bytes_written} bytes written");
        }
    }
}


fn main() {
    
    let listner = TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listner.incoming() {
        handle_client(stream.unwrap());
    }

}