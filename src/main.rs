use mysql::prelude::*;
use mysql::*;
use rpassword::prompt_password;
use std::io::{stdin, stdout, Write};

fn main() {
    // Get mysql conncection info from user
    let hostname = prompt_input("MySQL Hostname: ").unwrap();
    let port = prompt_input("MySQL Port Num: ").unwrap();
    let username = prompt_input("MySQL Username: ").unwrap();
    let password = prompt_password("MySQL Password: ").unwrap();
    let database = prompt_input("MySQL Database: ").unwrap();

    // Connect to mysql server using connection info
    let url = format!("mysql://{username}:{password}@{hostname}:{port}/{database}");
    let pool = Pool::new(url.as_str()).unwrap();
    let mut conn = pool.get_conn().unwrap();
}

fn prompt_input(prompt: &str) -> std::io::Result<String> {
    print!("{prompt}");
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let len = input.trim_end_matches(&['\r', '\n']).len();
    input.truncate(len);
    Ok(input)
}
