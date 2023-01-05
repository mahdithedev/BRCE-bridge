
    use serde::Deserialize;
    use std::fs;

    #[derive(Deserialize)]
    pub struct Config {
        pub storage_file: String,
        white_list: Option<Vec<String>>,
        black_list: Option<Vec<String>>,
        pub key: String,
        pub protocol_version: String,
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

        pub fn defualt() -> Config {
            Config {
                storage_file:"profiles.json".to_string(),
                key:"key".to_string(),
                protocol_version:"latest".to_string(),
                white_list: None , black_list: None, 
            }
        }

        pub fn read(file_path: &str) -> Config {
            let s = fs::read_to_string(file_path).unwrap();
            toml::from_str(&s[..]).unwrap()
        }

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
