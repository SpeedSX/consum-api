use once_cell::sync::Lazy;
use std::env;

static DEFAULT_PORT: u16 = 3030;
static DEFAULT_CONNECTION_STRING: &str = "server=tcp:localhost\\SQLEXPRESS,1433;User=sa;Password=sas;Database=Consum";

pub struct Configuration {
}

impl Configuration {
    pub fn get_connection_string(&self) -> &str {
        &CONN_STR
    }

    pub fn get_port(&self) -> u16 {
        *PORT
    }
}

pub static service_config: Configuration = Configuration {};

static CONN_STR: Lazy<String> = Lazy::new(|| {
    env::var("CONSUM_CONNECTION_STRING")
        .unwrap_or_else(|_| String::from(DEFAULT_CONNECTION_STRING))
});

static PORT: Lazy<u16> = Lazy::new(|| {
    env::var("CONSUM_PORT")
        .map(|s| s.parse::<u16>().unwrap_or(DEFAULT_PORT))
        .unwrap_or_else(|_| DEFAULT_PORT)
});
