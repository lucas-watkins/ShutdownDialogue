#[derive(Default)]
pub struct ShutdownCommands{
    pub shutdown: String,
    pub sleep: String,
    pub lock: String,
    pub logoff: String,
    pub restart: String,
}

impl ShutdownCommands {
    fn new() -> ShutdownCommands {
        // Parse json here

        ShutdownCommands {
            shutdown: String::new(),
            sleep: String::new(),
            lock: String::new(),
            logoff: String::new(),
            restart: String::new(),
        }
    }
}
