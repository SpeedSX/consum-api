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

use tiberius::{Client, Config, error::Error};
use tokio::net::TcpStream;
use tokio_util::compat::Compat;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use std::future::Future;

#[derive(Clone, Debug)]
pub struct TiberiusConnection {
    config: Config,
}

impl TiberiusConnection {
    /// Create a new `TiberiusConnection`.
    pub fn new(config: Config) -> TiberiusConnection {
        TiberiusConnection { config }
    }
}

impl bb8::ManageConnection for TiberiusConnection {
    type Connection = Client<Compat<TcpStream>>;
    type Error = Error;

    fn connect(&self) -> impl Future<Output = Result<Self::Connection, Self::Error>> + Send {
        let config = self.config.clone();
        async move {
            use tiberius::SqlBrowser;

            let tcp = TcpStream::connect_named(&config).await?;
            tcp.set_nodelay(true)?;

            Client::connect(config, tcp.compat_write()).await
        }
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            //debug!("Checking {:?}", conn);
            conn.simple_query("").await?.into_row().await?;
            Ok(())
        }
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}