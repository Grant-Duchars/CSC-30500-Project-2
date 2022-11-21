// Author: Grant Duchars
use mysql::prelude::*;
use mysql::*;
use std::io::{Error, ErrorKind};

struct Course {
    pub prefix: String,
    pub number: String,
    pub title: String,
    pub credits: String,
}

struct Grade {
    pub letter: String,
    pub value: String,
}

struct Semester {
    pub code: String,
    pub year: String,
    pub description: String,
}

struct Student {
    pub lname: String,
    pub fname: String,
    pub phone: String,
}

struct TakenCourse {
    pub student_lname: String,
    pub student_fname: String,
    pub course_prefix: String,
    pub course_number: String,
    pub grade_letter: String,
    pub semester_code: String,
}

pub fn setup_database(conn: &mut PooledConn) -> Result<()> {
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS Course (
        Prefix nvarchar(5) not null,
        Number int not null,
        Title nvarchar(32) not null,
        Credits int not null,
        PRIMARY KEY (Prefix, Number))",
    )?;
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS Grade (
        Letter nvarchar(5) not null,
        Value float not null,
        PRIMARY KEY (Letter))",
    )?;
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS Semester (
        Code nvarchar(5) not null,
        Year int not null,
        Description varchar(10) not null,
        PRIMARY KEY (Code))",
    )?;
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS Student (
        LName nvarchar(15) not null,
        FName nvarchar(15) not null,
        Phone nvarchar(20) not null,
        PRIMARY KEY (LName, FName))",
    )?;
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS TakenCourse (
        StudentLName nvarchar(32) not null,
        StudentFName nvarchar(32) not null,
        CoursePrefix nvarchar(5) not null,
        CourseNumber int not null,
        GradeLetter nvarchar(5) not null,
        SemesterCode nvarchar(5) not null,
        PRIMARY KEY (StudentLName, StudentFName, CoursePrefix, CourseNumber, GradeLetter, SemesterCode))",
    )?;
    Ok(())
}

// Start insert functions
/// Function to insert items into the database. Possible items to insert are courses, grades, semesters, students, and taken courses.
pub fn insert_into_database(conn: &mut PooledConn, input: Vec<&str>) -> Result<String> {
    let item = match input.get(1) {
        Some(&"c") => insert_course(
            conn,
            Course {
                prefix: match input.get(2) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Course needs a prefix.\n"
                    )))
                },
                number: match input.get(3) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Course needs a number.\n"
                    )))
                },
                title: match input.get(4) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Course needs a title.\n"
                    )))
                },
                credits: match input.get(5) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Course needs number of credits.\n"
                    )))
                },
            },
        )?,
        Some(&"g") => insert_grade(
            conn,
            Grade {
                letter: match input.get(2) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Grade needs a type.\n"
                    )))
                },
                value: match input.get(3) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Grade needs a point value.\n"
                    )))
                },
            },
        )?,
        Some(&"m") => insert_semester(
            conn,
            Semester {
                code: match input.get(2) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Semester needs a code.\n"
                    )))
                },
                year: match input.get(3) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Semester needs a year.\n"
                    )))
                },
                description: match input.get(4) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Semester needs a description.\n"
                    )))
                },
            },
        )?,
        Some(&"s") => insert_student(
            conn,
            Student {
                lname: match input.get(2) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Student needs a last name.\n"
                    )))
                },
                fname: match input.get(3) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Student needs a first name.\n"
                    )))
                },
                phone: match input.get(4) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Student needs a phone number.\n"
                    )))
                },
            },
        )?,
        Some(&"t") => insert_taken_course(
            conn,
            TakenCourse {
                student_lname: match input.get(2) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Taken course needs a student's last name.\n"
                    )))
                },
                student_fname: match input.get(3) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Taken course needs a student's first name.\n"
                    )))
                },
                course_prefix: match input.get(4) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Taken course needs a course's prefix.\n"
                    )))
                },
                course_number: match input.get(5) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Taken course needs a course's number.\n"
                    )))
                },
                grade_letter: match input.get(6) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Taken course needs a grade type.\n"
                    )))
                },
                semester_code: match input.get(7) {
                    Some(item) => item.to_string(),
                    None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other, 
                        "Error: Unable to insert item. Taken course needs a semester code.\n"
                    )))
                },
            },
        )?,
        _ => return Err(mysql::Error::IoError(Error::new(
            ErrorKind::Other, 
            "Error: Invalid subcommand. Valid subcommands are (c)ourse, (g)rade, se(m)ester, (s)tudent, and (t)aken course.\n"
        ))),
    };
    Ok(item)
}

fn insert_course(conn: &mut PooledConn, course: Course) -> Result<String> {
    let stmt = conn.prep(
        r"INSERT INTO Course (Prefix, Number, Title, Credits)
        VALUES (?, ?, ?, ?)",
    )?;
    conn.exec_drop(
        stmt,
        (
            &course.prefix,
            &course.number,
            &course.title,
            &course.credits,
        ),
    )?;
    Ok(format!(
        "{} {} {} {}",
        course.prefix, course.number, course.title, course.credits
    ))
}

fn insert_grade(conn: &mut PooledConn, grade: Grade) -> Result<String> {
    let stmt = conn.prep(
        r"INSERT INTO Grade (Letter, Value)
        VALUES (?, ?)",
    )?;
    conn.exec_drop(stmt, (&grade.letter, &grade.value))?;
    Ok(format!("{} {}", grade.letter, grade.value))
}

fn insert_semester(conn: &mut PooledConn, semester: Semester) -> Result<String> {
    let stmt = conn.prep(
        r"INSERT INTO Semester (Code, Year, Description)
        VALUES (?, ?, ?)",
    )?;
    conn.exec_drop(
        stmt,
        (&semester.code, &semester.year, &semester.description),
    )?;
    Ok(format!(
        "{} {} {}",
        semester.code, semester.year, semester.description
    ))
}

fn insert_student(conn: &mut PooledConn, student: Student) -> Result<String> {
    let stmt = conn.prep(
        r"INSERT INTO Student (LName, FName, Phone)
        VALUES (?, ?, ?)",
    )?;
    conn.exec_drop(stmt, (&student.lname, &student.fname, &student.phone))?;
    Ok(format!(
        "{} {} {}",
        student.lname, student.fname, student.phone
    ))
}

fn insert_taken_course(conn: &mut PooledConn, taken_course: TakenCourse) -> Result<String> {
    let stmt = conn.prep(
        r"INSERT INTO TakenCourse (
            StudentLName, 
            StudentFName, 
            CoursePrefix, 
            CourseNumber, 
            GradeLetter, 
            SemesterCode)
        VALUES (?, ?, ?, ?, ?, ?)",
    )?;
    conn.exec_drop(
        stmt,
        (
            &taken_course.student_lname,
            &taken_course.student_fname,
            &taken_course.course_prefix,
            &taken_course.course_number,
            &taken_course.grade_letter,
            &taken_course.semester_code,
        ),
    )?;
    Ok(format!(
        "{} {} {} {} {} {}",
        taken_course.student_lname,
        taken_course.student_fname,
        taken_course.course_prefix,
        taken_course.course_number,
        taken_course.grade_letter,
        taken_course.semester_code,
    ))
}
// End insert functions

// Start delete functions
/// Function to delete items from the database. Possible items to delete are students.
pub fn delete_from_database(conn: &mut PooledConn, input: Vec<&str>) -> Result<()> {
    delete_student(
        conn,
        Student {
            lname: match input.get(2) {
                Some(item) => item.to_string(),
                None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other,
                        "Error: Unable to delete student. No student supplied.\n",
                    )))
            },
            fname: match input.get(3) {
                Some(item) => item.to_string(),
                None => return Err(mysql::Error::IoError(Error::new(
                    ErrorKind::Other, 
                    "Error: Unable to delete student. Student's first name not supplied.\n"
                )))
            },
            phone: String::new(),
        },
    )?;
    Ok(())
}

fn delete_student(conn: &mut PooledConn, student: Student) -> Result<()> {
    let stmt = conn.prep(
        r"DELETE FROM Student
        WHERE Student.LName = ? AND Student.FName = ?",
    )?;
    conn.exec_drop(stmt, (&student.lname, &student.fname))?;
    let stmt = conn.prep(
        r"DELETE FROM TakenCourse
        WHERE TakenCourse.StudentLName = ? AND TakenCourse.StudentFName = ?",
    )?;
    conn.exec_drop(stmt, (student.lname, student.fname))?;
    Ok(())
}
// End delete functions


// Start list functions
/// Function to list items from the database. Possible items to list are courses, grades, semesters, students, and taken courses.
pub fn list_from_database(conn: &mut PooledConn, input: Vec<&str>) -> Result<()> {
    match input.get(1) {
        Some(&"c") => list_courses(conn)?,
        Some(&"g") => list_grades(conn)?,
        Some(&"m") => list_semesters(conn)?,
        Some(&"s") => list_students(conn)?,
        Some(&"t") => list_taken_courses(conn)?,
        _ => println!("Error: Invalid subcommand. Valid subcommands are (c)ourse, (g)rade, se(m)ster, (s)tudent, (t)aken course.\n"),
    }
    Ok(())
}

fn list_courses(conn: &mut PooledConn) -> Result<()> {
    let query = conn.query_map(
        r"SELECT * FROM Course",
        |(prefix, number, title, credits)| Course {
            prefix,
            number,
            title,
            credits,
        },
    )?;
    println!("+--------+--------+----------------------------------+---------+");
    println!("| Prefix | Number | Title                            | Credits |");
    println!("+--------+--------+----------------------------------+---------+");
    for item in query.iter() {
        println!(
            "| {:<6} | {:>6} | {:<32} | {:>7} |",
            item.prefix, item.number, item.title, item.credits
        );
    }
    println!("+--------+--------+----------------------------------+---------+\n");
    Ok(())
}

fn list_grades(conn: &mut PooledConn) -> Result<()> {
    let query = conn.query_map(r"SELECT * FROM Grade", |(letter, value)| Grade {
        letter,
        value,
    })?;
    println!("+------+-------+");
    println!("+ Type | Value |");
    println!("+------+-------+");
    for item in query.iter() {
        println!("| {:<4} | {:>5} |", item.letter, item.value);
    }
    println!("+------+-------+\n");
    Ok(())
}

fn list_semesters(conn: &mut PooledConn) -> Result<()> {
    let query = conn.query_map(r"SELECT * FROM Semester", |(code, year, description)| {
        Semester {
            code,
            year,
            description,
        }
    })?;
    println!("+------+------+--------+");
    println!("| Code | Year | Desc   |");
    println!("+------+------+--------+");
    for item in query.iter() {
        println!("| {:<4} | {:>4} | {:<6} |", item.code, item.year, item.description);
    }
    println!("+------+------+--------+\n");
    Ok(())
}

fn list_students(conn: &mut PooledConn) -> Result<()> {
    let query = conn.query_map(r"SELECT * FROM Student", |(lname, fname, phone)| Student {
        lname,
        fname,
        phone,
    })?;
    println!("+-----------------+-----------------+----------------------+");
    println!("| Last Name       | First Name      | Phone Number         |");
    println!("+-----------------+-----------------+----------------------+");
    for item in query.iter() {
        println!("| {:<15} | {:<15} | {:>20} |", item.lname, item.fname, item.phone);
    }
    println!("+-----------------+-----------------+----------------------+\n");
    Ok(())
}

fn list_taken_courses(conn: &mut PooledConn) -> Result<()> {
    let query = conn.query_map(
        r"SELECT * FROM TakenCourse",
        |(
            student_lname,
            student_fname,
            course_prefix,
            course_number,
            grade_letter,
            semester_code,
        )| {
            TakenCourse {
                student_lname,
                student_fname,
                course_prefix,
                course_number,
                grade_letter,
                semester_code,
            }
        },
    )?;
    println!("+-----------------+-----------------+--------+--------+------+------+");
    println!("| Last Name       | First Name      | Prefix | Number | Type | Code |");
    println!("+-----------------+-----------------+--------+--------+------+------+");
    for item in query.iter() {
        println!(
            "| {:<15} | {:<15} | {:<6} | {:>6} | {:<4} | {:<4} |",
            item.student_lname,
            item.student_fname,
            item.course_prefix,
            item.course_number,
            item.grade_letter,
            item.semester_code
        )
    }
    println!("+-----------------+-----------------+--------+--------+------+------+\n");
    Ok(())
}
// End list functions
