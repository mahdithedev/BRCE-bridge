use crate::error;

    // enum represnting difrent packet types defined by the spec see
    // https://github.com/mahdithedev/BRCE-specification#packet-structure
    // for more information
#[derive(PartialEq , Eq , Clone , Debug)]
pub enum Packet {
    // packet opcodes
    PROF(String),
    INIT(String),
    CONN(String),
    LIS(String),
    INP(String),
    OUT(String),
    ACK(),
    ERR(error::NetworkErrorCode),
    Unknown(u8),
}

// returns the packet opcode
impl From<Packet> for u8 {
    fn from(p: Packet) -> Self {
        match p {
            Packet::PROF(_) => 0,
            Packet::INIT(_) => 1,
            Packet::CONN(_) => 2,
            Packet::LIS(_) => 3,
            Packet::INP(_) => 4,
            Packet::OUT(_) => 5,
            Packet::ACK() => 6,
            Packet::ERR(_) => 7,
            Packet::Unknown(v) => v,
        }
    }
}

impl From<&Packet> for u8 {
    fn from(code: &Packet) -> Self {
        match code {
            Packet::PROF(_) => 0,
            Packet::INIT(_) => 1,
            Packet::CONN(_) => 2,
            Packet::LIS(_) => 3,
            Packet::INP(_) => 4,
            Packet::OUT(_) => 5,
            Packet::ACK() => 6,
            Packet::ERR(_) => 7,
            Packet::Unknown(v) => *v,
        }
    }
}

impl Packet {

    // serialize the packet into its bytes representation (byte vector)
    // check https://github.com/mahdithedev/BRCE-specification#packet-structure for
    // more information
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            Packet::ERR(v) => vec![7u8 , 1u8 , 0u8 , (*v) as u8],
            Packet::ACK() => vec![6u8 , 1u8 , 0u8 , 0u8],
            Packet::Unknown(_) => {panic!("tried to serielize unknown packet")}
            Packet::PROF(v) | Packet::INIT(v) | Packet::CONN(v) |
            Packet::LIS(v) | Packet::INP(v) | Packet::OUT(v)  => {

                let payload_size = v.len();

                let mut buffer: Vec<u8> =
                    vec![
                    self.into(),
                    (payload_size & 255) as u8 ,
                    (payload_size >> 8) as u8
                    ];

                buffer.extend_from_slice(v.as_bytes());
        
                return buffer;

            }
        }
    }

    pub fn deserilize(packet_type: u8 , payload: String , err: Option<error::NeErCode>) -> Packet {
        match packet_type {
            0 => Packet::PROF(String::from(payload)),
            1 => Packet::INIT(String::from(payload)),
            2 => Packet::CONN(String::from(payload)),
            3 => Packet::LIS(String::from(payload)),
            4 => Packet::INP(String::from(payload)),
            5 => Packet::OUT(String::from(payload)),
            6 => Packet::ERR(err.unwrap()),
            v => Packet::Unknown(v)
        }
    }

    pub fn deserilize_from_utf8(
        packet_type: u8,
        payload: &[u8], 
        err: Option<error::NeErCode>) -> Packet {

        // copy the payload from the buffer into an owned string
        let payload = String::from_utf8(payload.to_vec()).unwrap();

        match packet_type {
            0 => Packet::PROF(payload),
            1 => Packet::INIT(payload),
            2 => Packet::CONN(payload),
            3 => Packet::LIS(payload),
            4 => Packet::INP(payload),
            5 => Packet::OUT(payload),
            6 => Packet::ERR(err.unwrap()),
            v => Packet::Unknown(v)
        }
    }
    
    // parse the header section of a packet
    pub fn parse_header(buffer: &[u8]) -> Result<(u8 , usize) , error::ApplicationError> {

        if buffer.len() < 3 {
            return Err(error::ApplicationError(error::ApErCode::BYTESNOTENOUGH));
        }

        let packet_type = buffer[0];
        let payload_size = &buffer[1..3];

        let payload_size: u16 = (payload_size[0] | payload_size[1]<<7) as u16;

        Ok((packet_type , payload_size as usize))
        
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn serialization_test() {

        let mut buffer = vec![0u8 , 5u8 , 0u8];
        buffer.extend_from_slice("id432".as_bytes());  

        assert_eq!(
        buffer, 
            Packet::PROF(String::from("id432")).serialize()
        );

        let s: String = vec!['s';270].into_iter().collect();

        let mut buffer = vec![0u8 , 14u8 , 1u8];
        buffer.extend_from_slice(s.as_bytes());

        assert_eq!(
            buffer, 
            Packet::PROF(s).serialize()
        );
        
    }

    #[test]
    fn deserialization_test() {

        assert_eq!(Packet::deserilize(4 , String::from("ls -l") , None) , 
        Packet::INP(String::from("ls -l")) );

        assert_eq!(Packet::deserilize(6, String::from("") , 
        Some(error::NetworkErrorCode::AccessDenied)),
        Packet::ERR(error::NetworkErrorCode::AccessDenied))

    } 

}