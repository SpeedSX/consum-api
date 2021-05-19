# consum-api
![Rust](https://github.com/SpeedSX/consum-api/workflows/Rust/badge.svg?branch=master)
[![Gitpod ready-to-code](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/SpeedSX/consum-api)
## Description
Web API for Consum application database, implemented in [Rust](https://www.rust-lang.org/). 
Uses [tiberius](https://github.com/prisma/tiberius) for MS SQL Server access, [warp](https://github.com/seanmonstar/warp) as web framework.

This work is being done primarily for learning Rust, test/proof of using Rust for line-of-business application development, and in case of success, will be used for syncronization with accounting system.

See [issues](https://github.com/SpeedSX/consum-api/issues) for the list and description of features which are currently in development.

## Target platforms
Aimed to run on both Windows and Linux, tested on Windows only so far.

## Running the application
- Clone repository
- `SET CONSUM_ADDR=192.168.0.1:8080` or whatever needed (default is 127.0.0.1:3030)
- `SET CONSUM_CONNECTION_STRING=connection_string`, where connection_string to MSSQL DB is like `Server=ServerName;Database=Consum;User=Username;Password=Pa2386274`. *DB must exist, script is not included!*
- `SET CONSUM_MAX_POOL=10` - database connection pool size (default is 10)
- `SET CONSUM_STDOUT=true|false` - defines if log should write to console (default is true)
- `SET CONSUM_LOG_PATH=path|default` - specifies log path (must be full) or 'default' to write to default file name
- `SET CONSUM_JWT_SECRET=token` - optional, specifies JWT secret value for API key
- `cargo run --release`

## Running as Windows service
- Needs to be built with feature flag `cargo build --release --features "run-windows-service"`
- `sc create PolyConsService binPath=full_path_to_executable`
- `sc start PolyConsService`
