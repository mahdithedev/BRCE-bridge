use crate::error::{ErrorCode};


    #[derive(PartialEq , Eq , Clone , Debug)]
    pub enum Packet {
        // packet opcodes
        PROF(String),
        INIT(String),
        CONN(String),
        LIS(String),
        INP(String),
        OUT(String),
        ERR(ErrorCode),
    }

    impl From<Packet> for u8 {
        fn from(code: Packet) -> Self {
            match code {
                Packet::PROF(_) => 0,
                Packet::INIT(_) => 1,
                Packet::CONN(_) => 2,
                Packet::LIS(_) => 3,
                Packet::INP(_) => 4,
                Packet::OUT(_) => 5,
                Packet::ERR(_) => 6,
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
                Packet::ERR(_) => 6,
            }
        }
    }

    impl Packet {

        pub fn serialize(&self) -> Vec<u8> {
            match self {
                Packet::ERR(v) => vec![7u8 , 1u8 , 0u8 , (*v) as u8],
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
    }
