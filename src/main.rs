use std::{net , io::{Read, Write}};
use bridge::{packet::Packet, config::Config, profile , error};

fn get_packet(mut stream: &net::TcpStream) -> Result<Packet , error::ApplicationErrorCode> {

    const MAXIMUM_BUFFER_SIZE: usize = std::u16::MAX as usize;
    let mut buffer = [0u8;MAXIMUM_BUFFER_SIZE];
    let mut cursor = -1i32;

    if let Ok(bytes_read) = stream.read(&mut buffer[0..3]) {

        if bytes_read == 0 {
            return Err(error::ApplicationErrorCode::COMMUNACATIONCLOSED);
        }

        cursor += bytes_read as i32;

    } else {return Err(error::ApplicationErrorCode::COMMUNACATIONCLOSED);};

    let (packet_type , payload_size) = Packet::parse_header(&buffer[0..3]).unwrap();
    let mut total_bytes_read: usize = 0;

    let buffer_slice = &mut buffer[(cursor as usize)..payload_size+(cursor as usize)];

    while let Ok(bytes_read) = stream.read(buffer_slice) {

        if bytes_read == 0 {
            return Err(error::ApplicationErrorCode::COMMUNACATIONCLOSED);
        }

        cursor += bytes_read as i32;
        total_bytes_read += bytes_read;

        if total_bytes_read < payload_size as usize {continue}

        break;

    }

    let payload = &buffer[2..payload_size+2];

    let recieved_packet = Packet::deserilize_from_utf8(packet_type, payload, None);

    Ok(recieved_packet)

}

fn handle_client(mut stream: net::TcpStream) {

    let config = Config::default();
    let mut storage = profile::Storage::parse_file(&config.storage_file);

    loop {

        let packet = match get_packet(&stream) {
            Ok(v) => v,
            Err(_) => {
                stream.flush().unwrap();
                return;
            },
        };

        let response = match packet {
            Packet::LIS(v) => {
                let matches = storage.match_pattern(v); 
                Packet::LIS(matches.to_string())
            },
            Packet::PROF(prof) => {
                let prof: profile::Profile = serde_json::from_str(&prof).unwrap(); 
                storage.push(prof);
                storage.save(&config.storage_file);
                Packet::ACK()
            },
            _ => Packet::ACK()
        };

        if let Ok(_bytes_written) = stream.write(&response.serialize()) {
            continue;
        } else {
            return;
        }

    }

}

fn setup() {

    let listner = net::TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listner.incoming() {
        handle_client(stream.unwrap());
    }

}

fn main() {

    setup();

}