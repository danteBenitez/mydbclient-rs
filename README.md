# Simple MySQL DB client CLI (WIP)

This simple CLI connects to a MySQL database and allows you to run queries and see the results.
It implements a very simplified version of the MySQL protocol, and is not intended to be used in production.

## Usage

The CLI reads the config values from a TOML file that you pass as a parameter. The file should look like this:

```toml
username = "username"
password = "password"
db_name  = "test"
host  = "127.0.0.1"
port = 3306
```

## Progress

- [X] Read configuration file and print errors when needed.
    - [X] Read username, password, db_name, host and port.
- [X] Connect to the database via TCP/IP's abstraction (`std::net::TcpStream` in Rust).
- [] Complete handshake with the server.
    - [] Send authentication to the server.
- [] Send a query to the server.
- [] Parse the result and print it to the console.
