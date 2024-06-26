

pub struct Config {
    pub import_path: String,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        // Load configuration from file or environment variables
        Config {
            import_path: String::from("./import"),
            database_url: String::from("fpdb.db"),
        }
    }
}