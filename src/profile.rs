use serde::{Deserialize, Serialize};


#[derive(Serialize , Deserialize , Debug , Clone , PartialEq , Eq)]
pub struct Profile {
    pub id: String,
    pub os: String,
    pub os_version: String,
    pub interpeter: String,
    pub system_type: String,
    pub who:String,
}

impl Profile {
    pub fn new(
        id: String, 
        os: String,
        os_version: String,
        interpeter: String,
        system_type: String,
        who: String
        ) -> Profile {
        Profile {id , os , os_version , interpeter , system_type , who}
    }
}

#[derive(Deserialize , Debug , Clone , PartialEq , Eq)]
pub struct Storage {
    pub list: Vec<Profile>
}

impl Storage {

    pub fn parse_file(path: String) -> Storage {
        let profiles = std::fs::read_to_string(path).unwrap();
        let storage: Storage = serde_json::from_str(&profiles).unwrap();
        storage
    }

    pub fn match_pattern(&self , pattern: String) -> String {
        for profile in &self.list {
            // for now we will only match for equality and more advanced patterns wil not be
            // handeled for now
            if profile.id == pattern {
                return serde_json::to_string(&profile).unwrap();
            }
        }
        String::from("")
    }

}