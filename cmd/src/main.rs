use db_client::{DbClient, DbConfigOptions};
use std::io::{Read, Write};
use std::{env, process::exit};

fn usage(args: Vec<String>) {
    println!("Usage: {} <path-to-config-file>.toml", args[0]);
    println!("  path-to-config-file: Path to the configuration file for the database connection and auth");
    println!(
        "Simple MySQL client that reads a configuration file (TOML) and connects to the database."
    );
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        usage(args);
        return;
    }

    let config_file = &args[1];
    let config = DbConfigOptions::from_toml(config_file).unwrap_or_else(|err| {
        eprintln!("[ERROR]: {}", err);
        usage(args);
        exit(1);
    });
    let mut client = DbClient::new(config);
    client.connect().unwrap_or_else(|err| {
        eprintln!("[ERROR]: {}", err);
        exit(1);
    });
    let conn = client.get_connection().unwrap_or_else(|| {
        eprintln!("[ERROR]: Failed to get connection to the database");
        exit(1);
    });
    
    conn
        .bytes()
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .for_each(|b| {
            std::io::stdout().flush().unwrap();
            print!("{}", b);
        });
    // let string = String::from_utf8_lossy(&result);

    // println!("Result from the server: {:?}", result);
    // println!("Result from the server (as string): {:?}", string);
}
