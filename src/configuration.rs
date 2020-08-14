use once_cell::sync::Lazy;
use std::{net::{IpAddr, SocketAddr, Ipv4Addr}, env};

//static DEFAULT_HOST: [u32; 4] = [127, 0, 0, 1];
const DEFAULT_PORT: u16 = 3030;
static DEFAULT_CONNECTION_STRING: &str = "server=tcp:localhost\\SQLEXPRESS,1433;User=sa;Password=sas;Database=Consum";
const DEFAULT_MAX_POOL: u32 = 10;
const DEFAULT_STDOUT: bool = true;
const DEFAULT_LOG_NAME: &str = "output.log";
const DEFAULT_JWT_SECRET: &str = "consum_jwt_secret";

pub struct Configuration {
}

impl Configuration {
    pub fn get_connection_string(&self) -> &str {
        &CONN_STR
    }

    pub fn get_max_pool(&self) -> u32 {
        *MAX_POOL
    }

    pub fn get_addr(&self) -> SocketAddr {
        *ADDR
    }

    pub fn stdout_enabled(&self) -> bool {
        *STDOUT
    }

    pub fn get_log_path(&self) -> Option<&String> {
        LOG_PATH.as_ref()
    }

    pub fn get_jwt_secret(&self) -> &str {
        &JWT_SECRET
    }
}

pub static SERVICE_CONFIG: Configuration = Configuration {};

static CONN_STR: Lazy<String> = Lazy::new(|| {
    env::var("CONSUM_CONNECTION_STRING")
        .unwrap_or_else(|_| String::from(DEFAULT_CONNECTION_STRING))
});

static MAX_POOL: Lazy<u32> = Lazy::new(|| {
    env::var("CONSUM_MAX_POOL")
        .map(|s| s.parse::<u32>().unwrap_or(DEFAULT_MAX_POOL))
        .unwrap_or(DEFAULT_MAX_POOL)
});

static ADDR: Lazy<SocketAddr> = Lazy::new(|| {
    let default: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), DEFAULT_PORT);
    env::var("CONSUM_ADDR")
        .map(|s| s.parse::<SocketAddr>().unwrap_or(default))
        .unwrap_or_else(|_| default)
});

static STDOUT: Lazy<bool> = Lazy::new(|| {
    env::var("CONSUM_STDOUT")
        .map(|s| s.parse::<bool>().unwrap_or(DEFAULT_STDOUT))
        .unwrap_or(DEFAULT_STDOUT)
});

static LOG_PATH: Lazy<Option<String>> = Lazy::new(|| {
    env::var("CONSUM_LOG_PATH")
        .map(|path|
            if path.to_uppercase() == "DEFAULT" {  
                env::current_exe()
                    .map(|dir| dir.as_path().with_file_name(DEFAULT_LOG_NAME).to_string_lossy().to_string())
                    .unwrap_or(path)
            } else {
                path 
            })
        .ok()
});

static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    env::var("CONSUM_JWT_SECRET")
        .unwrap_or_else(|_| String::from(DEFAULT_JWT_SECRET))
});