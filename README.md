# consum-api
## Description
Web API for Consum application database, implemented in [Rust](https://www.rust-lang.org/). 
Uses [tiberius](https://github.com/prisma/tiberius) for MS SQL Server access, [warp](https://github.com/seanmonstar/warp) as web framework.

This work is being done primarily for learning Rust, test/proof of using Rust for line-of-business application development, and in case of success, will be used for syncronization with accounting system.

See [issues](https://github.com/SpeedSX/consum-api/issues) for the list and description of features which are currently in development.

## Target platforms
Aimed to be run on both Windows and Linux, tested on Windows only so far.

## Running the application
- Clone repository
- `SET CONSUM_ADDR=192.168.0.1:8080` or whatever needed (default is 127.0.0.1:3030)
- `SET CONSUM_CONNECTION_STRING=connection_string`, where connection_string is like `Server=ServerName;Database=Consum;User=Username;Password=Pa2386274`.
- `cargo run --release`

## Building as Windows service
- `cargo build --release --features "run_windows_service"`
- `sc create PolyConsService binPath=full_path_to_executable`
- `sc start PolyConsService`
