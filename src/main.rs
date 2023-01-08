use std::{net::{TcpListener , TcpStream}, io::{Read, Write}};
use bridge::{config::Config, packet::Packet};

fn handle_client(mut stream: TcpStream) {

        // std::u16::MAX as usize
        const MAXIMUM_BUFFER_SIZE: usize = 30;
        let mut buffer = [0u8;MAXIMUM_BUFFER_SIZE];
        let mut cursor = -1i32;

        loop {

            if let Ok(bytes_read) = stream.read(&mut buffer[0..3]) {

                if bytes_read == 0 {
                    return;
                }

                cursor += bytes_read as i32;

            } else {return};

            let (packet_type , payload_size) = Packet::parse_header(&buffer[0..3]).unwrap();
            let mut total_bytes_read: usize = 0;

            let buffer_slice = &mut buffer[(cursor as usize)..payload_size+(cursor as usize)];

            while let Ok(bytes_read) = stream.read(buffer_slice) {

                if bytes_read == 0 {
                    return;
                }

                cursor += bytes_read as i32;
                total_bytes_read += bytes_read;

                if total_bytes_read < payload_size as usize {continue}

                break;

            }

            println!("{:?}" , buffer);

            let packet = 
            Packet::deserilize_from_utf8(
                packet_type, 
                &buffer[2..(payload_size as usize)+2],
                None);

            println!("{:?}" , packet);

            if let Ok(bytes_written) = stream.write(&packet.serialize()) {
                println!("{bytes_written} bytes written");
            }

            cursor = -1;

        }

}

fn setup() {
    let listner = TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listner.incoming() {
        handle_client(stream.unwrap());
    }
}

fn main() {
    setup();
}