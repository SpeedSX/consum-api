use once_cell::sync::Lazy;
use std::{net::{IpAddr, SocketAddr, Ipv4Addr}, env};

const DEFAULT_PORT: u16 = 3030;
static DEFAULT_CONNECTION_STRING: &str = "server=tcp:localhost,1433;TrustServerCertificate=true;User=sa;Password=sas;Database=Consum";
//"server=tcp:localhost,1433;IntegratedSecurity=true;TrustServerCertificate=true;Database=Consum";
const DEFAULT_MAX_POOL: u32 = 10;
const DEFAULT_STDOUT: bool = true;
const DEFAULT_LOG_NAME: &str = "output.log";
const DEFAULT_JWT_SECRET: &str = "consum_jwt_secret";

pub struct Configuration {
    f_connection_string: String,
    f_max_pool: u32,
    f_addr: SocketAddr,
    f_stdout_enabled: bool,
    f_log_path: Option<String>,
    f_jwt_secret: String,
}

impl Configuration {
    pub fn connection_string(&self) -> &str {
        &self.f_connection_string
    }

    pub fn max_pool(&self) -> u32 {
        self.f_max_pool
    }

    pub fn addr(&self) -> SocketAddr {
        self.f_addr
    }

    pub fn stdout_enabled(&self) -> bool {
        self.f_stdout_enabled
    }

    pub fn log_path(&self) -> Option<&String> {
        self.f_log_path.as_ref()
    }

    pub fn jwt_secret(&self) -> &str {
        &self.f_jwt_secret
    }
}

static SERVICE_CONFIG: Lazy<Configuration> = Lazy::new(|| {
    Configuration {
        f_connection_string: env::var("CONSUM_CONNECTION_STRING")
            .unwrap_or_else(|_| String::from(DEFAULT_CONNECTION_STRING)),
        f_max_pool: env::var("CONSUM_MAX_POOL")
            .map(|s| s.parse::<u32>().unwrap_or(DEFAULT_MAX_POOL))
            .unwrap_or(DEFAULT_MAX_POOL),
        f_addr: {
            let default: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), DEFAULT_PORT);
            env::var("CONSUM_ADDR")
                .map(|s| s.parse::<SocketAddr>().unwrap_or(default))
                .unwrap_or_else(|_| default)
        },
        f_stdout_enabled: env::var("CONSUM_STDOUT")
            .map(|s| s.parse::<bool>().unwrap_or(DEFAULT_STDOUT))
            .unwrap_or(DEFAULT_STDOUT),
        f_log_path: env::var("CONSUM_LOG_PATH")
            .map(|path|
                if path.to_uppercase() == "DEFAULT" {  
                    env::current_exe()
                        .map(|dir| dir.as_path().with_file_name(DEFAULT_LOG_NAME).to_string_lossy().to_string())
                        .unwrap_or(path)
                } else {
                    path 
                })
            .ok(),
        f_jwt_secret: env::var("CONSUM_JWT_SECRET")
            .unwrap_or_else(|_| String::from(DEFAULT_JWT_SECRET)),
    }
});

pub fn get() -> &'static Configuration {
    &SERVICE_CONFIG
}
