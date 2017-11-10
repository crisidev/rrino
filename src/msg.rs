pub mod kind {
    #[derive(Serialize, Deserialize, Debug)]
    pub struct NotifyMsg {
        title: String,
        message: String,
    }

    impl NotifyMsg {
        pub fn get_title(&self) -> &String {
            &self.title
        }

        pub fn get_message(&self) -> &String {
            &self.message
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ShutdownMsg {
        force: bool,
    }

    impl ShutdownMsg {
        pub fn is_forced(&self) -> bool {
            self.force
        }
    }
}
