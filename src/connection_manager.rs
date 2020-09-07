//! Tiberius support for the `bb8` connection pool.
//!
//! # Example
//!
//! ```
//! use futures::future::join_all;
//!
//! #[tokio::main]
//! async fn main() {
//!     let manager = TiberiusConnectionManager::new(Config::from_ado_string(SERVICE_CONFIG.get_connection_string()).unwrap()).unwrap();
//!     let pool = bb8::Pool::builder().build(manager).await.unwrap();
//!
//!     let mut handles = vec![];
//!
//!     for _i in 0..10 {
//!         let pool = pool.clone();
//!
//!         handles.push(tokio::spawn(async move {
//!             let mut conn = pool.get().await.unwrap();
//!
//!             let stream = client.simple_query("SELECT @@VERSION").await?;
//!             let rows: Vec<Row> = stream.into_first_result().await?;
//!         }));
//!     }
//!
//!     join_all(handles).await;
//! }
//! ```
#![allow(clippy::needless_doctest_main)]
#![deny(missing_docs, missing_debug_implementations)]

pub use bb8;
pub use tiberius;

use async_trait::async_trait;
use tiberius::{Client, Config, error::Error};
use tokio::net::TcpStream;
use tokio_util::compat::Compat;
use tokio_util::compat::Tokio02AsyncWriteCompatExt;

#[derive(Clone, Debug)]
pub struct TiberiusConnectionManager {
    config: Config,
}

impl TiberiusConnectionManager {
    /// Create a new `TiberiusConnectionManager`.
    pub fn new(config: Config) -> tiberius::Result<TiberiusConnectionManager> {
        
        Ok(TiberiusConnectionManager {
            config
        })
    }
}

#[async_trait]
impl bb8::ManageConnection for TiberiusConnectionManager {
    type Connection = Client<Compat<TcpStream>>;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {

        let tcp = TcpStream::connect(&self.config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        //debug!("Connecting...");

        Client::connect(self.config.clone(), tcp.compat_write()).await
    }

    async fn is_valid(&self, mut conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        //debug!("Checking {:?}", conn);
        conn.simple_query("").await?.into_row().await?;
        Ok(conn)
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}