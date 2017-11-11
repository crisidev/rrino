pub mod kind {
    #[derive(Serialize, Deserialize, Debug)]
    pub struct NotifyMsg {
        pub from: String,
        pub message: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ShutdownMsg {
        pub force: bool,
    }
}
