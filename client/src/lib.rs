#![allow(dead_code)]

use std::{
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
};
use toml::Table;

pub struct DbConfigOptions {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub db_name: String,
    pub host: String,
}

impl DbConfigOptions {
    pub fn from_toml(file_path: &str) -> Result<Self, &'static str> {
        let file = fs::read_to_string(file_path).map_err(|_| "could not read config file")?;
        let config = (&file)
            .parse::<Table>()
            .map_err(|_| "could not parse config file. Is it valid TOML?")?;

        let username = config
            .get("username")
            .ok_or("username required")?
            .as_str()
            .ok_or("username must be a string")?;

        let password = config
            .get("password")
            .ok_or("password required")?
            .as_str()
            .ok_or("password must be a string")?;

        let port = config
            .get("port")
            .ok_or("port required")?
            .as_integer()
            .ok_or("port must be an integer")?
            .try_into()
            .map_err(|_| "port must be u16. Range from 0 to ")?;

        let db_name = config
            .get("db_name")
            .ok_or("database name required")?
            .as_str()
            .ok_or("database name must be a string")?;

        let host = config
            .get("host")
            .ok_or("host required")?
            .as_str()
            .ok_or("host must be a string")?;

        let config = Self {
            username: username.to_owned(),
            password: password.to_owned(),
            port,
            db_name: db_name.to_owned(),
            host: host.to_owned(),
        };
        Ok(config)
    }
}

pub struct DbClient(DbConfigOptions, Option<TcpStream>);

impl DbClient {
    pub fn new(config: DbConfigOptions) -> Self {
        Self(config, None)
    }

    pub fn connect(&mut self) -> Result<(), String> {
        let ip: Ipv4Addr = self
            .0
            .host
            .parse()
            .map_err(|e| format!("could not parse host as IP: {}", e))?;
        let ip = IpAddr::V4(ip);
        let addr = SocketAddr::new(ip, self.0.port);
        let conn =
            TcpStream::connect(addr).map_err(|e| format!("could not connect to server: {}", e))?;
        self.1 = Some(conn);
        Ok(())
    }

    pub fn get_connection(self) -> Option<TcpStream> {
        self.1
    }
}
