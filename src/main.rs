// Author: Grant Duchars
mod dbmgmt;
use dbmgmt::*;
use mysql::*;
use rpassword::prompt_password;
use std::io::{stdin, stdout, Write};

// Need to implement:
// Add(Class, Grade, Semester, Student, Taken Class),
// List(Class, Grade, Semester, Student, Taken Class),
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

    println!("You are now connected to {hostname} using {database} database.");

    setup_database(&mut conn).unwrap();

    loop {
        let input = prompt_input(">>> ").unwrap();
        let input: Vec<&str> = input.split(" ").collect();
        match input[0] {
            "a" => insert_into_database(&mut conn, input).unwrap(),
            "d" => todo!(),
            "l" => todo!(),
            "t" => todo!(),
            "q" => return,
            _ => println!("Error: Invalid command. Valid commands are (a)dd, (d)elete, (l)ist, (t)ranscript, (q)uit."),
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

fn insert_into_database(conn: &mut PooledConn, input: Vec<&str>) -> Result<()> {
    match input[1] {
        "c" => insert_course(conn, Course{prefix: input[2], number: input[3], title: input[4], credits: input[5]})?,
        "g" => insert_grade(conn, Grade{letter: input[2], value: input[3]})?,
        "m" => insert_semester(conn, Semester{code: input[2], year: input[3], description: input[4]})?,
        "s" => insert_student(conn, Student{lname: input[2], fname: input[3], phone: input[4]})?,
        "t" => insert_taken_course(conn, TakenCourse{student_lname: input[2], student_fname: input[3], course_prefix: input[4], course_number: input[5], grade_letter: input[6], semester_code: input[7]})?,
        _ => println!("Error: Invalid subcommand. Valid subcommands are (c)ourse, (g)rade, se(m)ster, (s)tudent, (t)aken course"),
    }
    Ok(())
}
