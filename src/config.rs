
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    // the location of the json file
    pub storage_file: String,
    // alowed ip addresses
    white_list: Option<Vec<String>>,
    // blocked ip addresses
    black_list: Option<Vec<String>>,
    // coustome length and format security key for operations that require authorization
    pub key: String,
    pub protocol_version: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            storage_file:"profiles.json".to_string(),
            key:"key".to_string(),
            protocol_version:"latest".to_string(),
            white_list: None , black_list: None, 
        }
    }
}

impl Config {

    pub fn new(storage_file: &str, key: &str, protocol_version: &str ) -> Config {
        Config {
            storage_file: storage_file.to_string(),
            key: key.to_string(),
            protocol_version: protocol_version.to_string(),
            white_list: None , black_list: None
        }
    }

    // reads configuration from a toml file
    pub fn read(file_path: &str) -> Config {
        let s = fs::read_to_string(file_path).unwrap();
        toml::from_str(&s[..]).unwrap()
    }

    // checks if an ip address is alowed
    pub fn allowed(&self , ip: &String) -> bool {

        let white_list = &self.white_list;
        let black_list = &self.black_list;

        if let Some(list) = black_list {
            if list.contains(&ip) {
                return false;
            }
        }

        if let Some(list) = white_list {
            if !list.contains(&ip) {
                return false;
            }
        }

        true

    }

    pub fn add_ip_to_whitelist(&mut self , ip: &String) -> Result<() , &str> {

        if let None = &self.white_list {
            self.white_list = Some(vec![]);
        }

        if let Some(list) = &self.black_list {
            if list.contains(ip) {
                return Err("ip is already in blacklist");
            }
        }

        if let Some(list) = &mut self.white_list {
            list.push(ip.clone());
        }

        Ok(())
        
    }

    pub fn add_ip_to_blacklist(&mut self , ip: &String) -> Result<() , &str> {

        if let None = &self.black_list {
            self.black_list = Some(vec![]);
        }

        if let Some(list) = &self.white_list {
            if list.contains(ip) {
                return Err("ip is already in whitelist");
            }
        }

        if let Some(list) = &mut self.black_list {
            list.push(ip.clone());
        }

        Ok(())

    }

}

    
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ip_cant_be_in_both_lists() {
        
        let mut c = Config::default();
        let ip = String::from("127.0.0.1");

        c.add_ip_to_whitelist(&ip).unwrap();

        if let Ok(_) = c.add_ip_to_blacklist(&ip) {
            assert!(false)
        }

    }

    #[test]
    fn blacklisted_ip_not_allowed() {

        let mut c: Config = Config::default();
        let ip = String::from("127.0.0.1");

        c.add_ip_to_blacklist(&ip).unwrap();

        assert!(!c.allowed(&ip));

    }

    #[test]
    fn ip_not_in_whitelist_not_allowed() {

        let mut c = Config::default();
        let ip1 = String::from("127.0.0.1");
        let ip2 = String::from("197.8.2.3");

        c.add_ip_to_whitelist(&ip1).unwrap();

        assert!(!c.allowed(&ip2));

    }

}