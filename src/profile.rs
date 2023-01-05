
    pub struct Profile {
        pub id: String,
        pub os: String,
        pub os_version: String,
        pub interpeter: String,
        pub who:String,
    }

    impl Profile {
        pub fn new(
            id: String, 
            os: String,
            os_version: String,
            interpeter: String,
            who: String
            ) -> Profile {
            Profile {id , os , os_version , interpeter , who}
        }
    }
