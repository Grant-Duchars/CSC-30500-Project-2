// Author: Grant Duchars
mod dbmgmt;
use dbmgmt::*;
use mysql::*;
use rpassword::prompt_password;
use std::io::{stdin, stdout, ErrorKind, Write};

// Need to implement:
// Transcript
// Quit

fn main() {
    // Get mysql connection info from user
    let hostname = prompt_input("MySQL Hostname: ").unwrap();
    let port_num = prompt_input("MySQL Port Num: ").unwrap();
    let username = prompt_input("MySQL Username: ").unwrap();
    let password = prompt_password("MySQL Password: ").unwrap();
    let database = prompt_input("MySQL Database: ").unwrap();

    // Connect to mysql server using connection info
    let url = format!("mysql://{username}:{password}@{hostname}:{port_num}/{database}");
    let pool = Pool::new(url.as_str()).unwrap();
    let mut conn = pool.get_conn().unwrap();

    // Notify user that the connection has been made
    println!("\nYou are now connected to '{hostname}' using '{database}' database.\n");

    // Create the neccessary database tables if they do not exist
    setup_database(&mut conn).unwrap();

    // Main loop. Prompts user for input and terminates when user inputs "q"
    loop {
        let input = prompt_input(">>> ").unwrap();
        let input: Vec<&str> = input.split(" ").collect();
        // Check what command the user wants to run
        match input[0] {
            "a" => {
                // Run the insert and check if any mysql errors were thrown
                match insert_into_database(&mut conn, input) {
                    Err(e) => {
                        match e {
                            mysql::Error::MySqlError(my_sql_error) => {
                                if my_sql_error.code == 1062 {
                                    println!("Error: Unable to add item to database. Duplicate entry found.\n");
                                }
                            }
                            mysql::Error::IoError(error) => {
                                if error.kind() == ErrorKind::Other {
                                    println!("{error}");
                                }
                            },
                            _ => panic!(),
                        }
                    },
                    Ok(item) => println!("Successfully inserted '{item}' into database.\n")
                }
            }
            "d" => delete_from_database(&mut conn, input).unwrap(),
            "l" => list_from_database(&mut conn, input).unwrap(),
            "t" => transcript(&mut conn, input).unwrap(),
            "q" => return,
            _ => println!("Error: Invalid command. Valid commands are (a)dd, (d)elete, (l)ist, (t)ranscript, (q)uit.\n"),
        }
    }
}

fn prompt_input(prompt: &str) -> Result<String> {
    print!("{prompt}");
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let len = input.trim_end_matches(&['\r', '\n']).len();
    input.truncate(len);
    Ok(input)
}
