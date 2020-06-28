use once_cell::sync::Lazy;
use std::{net::{IpAddr, SocketAddr, Ipv4Addr}, env};

//static DEFAULT_HOST: [u32; 4] = [127, 0, 0, 1];
static DEFAULT_PORT: u16 = 3030;
static DEFAULT_CONNECTION_STRING: &str = "server=tcp:localhost\\SQLEXPRESS,1433;User=sa;Password=sas;Database=Consum";

pub struct Configuration {
}

impl Configuration {
    pub fn get_connection_string(&self) -> &str {
        &CONN_STR
    }

    pub fn get_addr(&self) -> SocketAddr {
        *ADDR
    }
}

pub static service_config: Configuration = Configuration {};

static CONN_STR: Lazy<String> = Lazy::new(|| {
    env::var("CONSUM_CONNECTION_STRING")
        .unwrap_or_else(|_| String::from(DEFAULT_CONNECTION_STRING))
});

static ADDR: Lazy<SocketAddr> = Lazy::new(|| {
    let default: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), DEFAULT_PORT);
    env::var("CONSUM_ADDR")
        .map(|s| s.parse::<SocketAddr>().unwrap_or(default))
        .unwrap_or_else(|_| default)
});
