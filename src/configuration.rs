use std::str::FromStr;
use std::sync::LazyLock;
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

const DEFAULT_PORT: u16 = 3030;
static DEFAULT_CONNECTION_STRING: &str = "server=tcp:localhost,1433;TrustServerCertificate=true;User=alexey;Password=dosia;Database=Consum";
//"server=tcp:localhost,1433;IntegratedSecurity=true;TrustServerCertificate=true;Database=Consum";
const DEFAULT_MAX_POOL: u32 = 10;
const DEFAULT_STDOUT: bool = true;
const DEFAULT_LOG_NAME: &str = "output.log";
const DEFAULT_JWT_SECRET: &str = "consum_jwt_secret";

pub struct Configuration {
    connection_string: String,
    max_pool: u32,
    addr: SocketAddr,
    stdout_enabled: bool,
    log_path: Option<String>,
    jwt_secret: String,
}

impl Configuration {
    pub fn connection_string(&self) -> &str {
        &self.connection_string
    }

    pub fn max_pool(&self) -> u32 {
        self.max_pool
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn stdout_enabled(&self) -> bool {
        self.stdout_enabled
    }

    pub fn log_path(&self) -> Option<&String> {
        self.log_path.as_ref()
    }

    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }
}
static SERVICE_CONFIG: LazyLock<Configuration> = LazyLock::new(|| Configuration {
    connection_string: get_env_var_or_default(
        "CONSUM_CONNECTION_STRING",
        DEFAULT_CONNECTION_STRING.to_string(),
    ),
    max_pool: get_env_var_or_default("CONSUM_MAX_POOL", DEFAULT_MAX_POOL),
    addr: get_socket_address(),
    stdout_enabled: get_env_var_or_default("CONSUM_STDOUT", DEFAULT_STDOUT),
    log_path: get_log_path(),
    jwt_secret: get_env_var_or_default("CONSUM_JWT_SECRET", DEFAULT_JWT_SECRET.to_string()),
});

// Helper function to retrieve an environment variable or use a default value.
fn get_env_var_or_default<T: FromStr>(key: &str, default: T) -> T
where
    T::Err: std::fmt::Debug,
{
    env::var(key)
        .ok()
        .and_then(|value| value.parse::<T>().ok())
        .unwrap_or(default)
}

// Extracted function for determining the SocketAddr
fn get_socket_address() -> SocketAddr {
    let default_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), DEFAULT_PORT);
    env::var("CONSUM_ADDR")
        .map(|s| s.parse::<SocketAddr>().unwrap_or(default_addr))
        .unwrap_or(default_addr)
}

// Extracted function for handling log path logic
fn get_log_path() -> Option<String> {
    env::var("CONSUM_LOG_PATH").ok().map(|path| {
        if path.to_uppercase() == "DEFAULT" {
            env::current_exe()
                .map(|dir| {
                    dir.as_path()
                        .with_file_name(DEFAULT_LOG_NAME)
                        .to_string_lossy()
                        .to_string()
                })
                .unwrap_or(path)
        } else {
            path
        }
    })
}

pub fn get() -> &'static Configuration {
    &SERVICE_CONFIG
}
