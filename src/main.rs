use std::{net::{TcpListener , TcpStream}, io::{Read, Write}};
use bridge::{config::Config, packet::Packet , parser};

fn read_parse_header(stream: &mut TcpStream) -> (u8 , usize) {

    let mut buffer = [0u8;3];
    let mut start = 0;

    while let Ok(bytes_read) = stream.read(&mut buffer[start..]) {
        
        if bytes_read == 0 {
            continue;
        }

        if bytes_read < 3 {
            start += bytes_read;
            continue;
        }

        let r = parser::parse_header(&buffer[0..]).unwrap();
        return (r.0 , r.1 as usize); 
    }

    (0 , 0)

}

fn handle_client(mut stream: TcpStream) {

        let (packet_type , payload_size) = read_parse_header(&mut stream);
        const MAXIMUM_BUFFER_SIZE: usize = std::u16::MAX as usize;
        let mut buffer = [0u8;MAXIMUM_BUFFER_SIZE];

        let mut start = 3;
    

        while let Ok(bytes_read) = stream.read(&mut buffer[start..payload_size+3 as usize]) {
            if bytes_read == 0 {
                println!("bytes read are 0");
                continue;
            }
            if bytes_read < payload_size {
                start += bytes_read;
                continue;
            } 
            println!("number of bytes read = {bytes_read} , payload size = {payload_size}");
            break;
        }

        println!("reached here");

        let packet = 
        Packet::deserilize_fro_utf8(packet_type , &buffer[start..payload_size+3] , None);

        println!("{:?}" , packet);

        if let Ok(bytes_written) = stream.write(&packet.serialize()) {
            println!("{bytes_written} bytes written");
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