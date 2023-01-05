pub mod config;
pub mod profile;
pub mod error;
pub mod packet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ip_cant_be_in_both_lists() {
        
        let mut c = config::Config::defualt();
        let ip = String::from("127.0.0.1");

        c.add_ip_to_whitelist(&ip).unwrap();

        if let Ok(_) = c.add_ip_to_blacklist(&ip) {
            assert!(false)
        }

    }

    #[test]
    fn blacklisted_ip_not_allowed() {

        let mut c = config::Config::defualt();
        let ip = String::from("127.0.0.1");

        c.add_ip_to_blacklist(&ip).unwrap();

        assert!(!c.allowed(&ip));

    }

    #[test]
    fn ip_not_in_whitelist_not_allowed() {

        let mut c = config::Config::defualt();
        let ip1 = String::from("127.0.0.1");
        let ip2 = String::from("197.8.2.3");

        c.add_ip_to_whitelist(&ip1).unwrap();

        assert!(!c.allowed(&ip2));

    }

    #[test]
    fn binary_packet_test() {

        let mut buffer = vec![0u8 , 5u8 , 0u8];
        buffer.extend_from_slice("id432".as_bytes());  

        assert_eq!(
        buffer, 
        packet::Packet::PROF(String::from("id432")).serialize()
        );

        let s: String = vec!['s';270].into_iter().collect();

        let mut buffer = vec![0u8 , 14u8 , 1u8];
        buffer.extend_from_slice(s.as_bytes());

        assert_eq!(
            buffer, 
            packet::Packet::PROF(s).serialize()
        );
        
    }

}