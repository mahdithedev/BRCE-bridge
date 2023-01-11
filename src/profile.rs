use serde::{Deserialize, Serialize};

#[derive(Serialize , Deserialize , Debug , Clone , PartialEq , Eq)]
pub struct Profile {
    pub id: String,
    pub platform: String,
    pub platform_release: String,
    pub platform_version: String,
    pub interpeter: String,
    pub hostname: String,
    pub ip_address: String,
    pub mac_address: String,
    pub processor: String,
    pub ram: String,
    pub architecture: String,
    pub who:String,
}

// impl Profile {
//     pub fn new(
//         id: String, 
//         platform: String,
//         platform_version: String,
//         interpeter: String,
//         architecture: String,
//         platform_release
//         who: String
//         ) -> Profile {
//         Profile 
//         {id , platform , platform_version , interpeter , architecture , who , platform_release}
//     }
// }

#[derive(Serialize , Deserialize , Debug , Clone , PartialEq , Eq)]
pub struct Storage {
    pub list: Vec<Profile>
}

impl Storage {

    pub fn parse_file(path: &String) -> Storage {
        let profiles = std::fs::read_to_string(path).unwrap();
        let storage: Storage = serde_json::from_str(&profiles).unwrap();
        storage
    }

    pub fn match_pattern(&self , pattern: String) -> String {
        let mut filtered: Vec<&Profile> = vec![];
        for profile in &self.list {
            // for now we will only match for equality and the star wildcard.
            // more advanced patterns wil not be
            // handeled for now
            if profile.id == pattern || pattern == "*" {
                filtered.push(&profile);
            }
        }
        return  serde_json::to_string(&filtered).unwrap();
    }

    pub fn push(&mut self , profile: Profile) {
        self.list.push(profile);
    }

    pub fn save(&self , path: &String) {
        std::fs::write(path, serde_json::to_string_pretty(&self).unwrap()).unwrap();
    }

}