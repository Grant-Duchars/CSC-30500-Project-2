// Author: Grant Duchars
use mysql::prelude::*;
use mysql::*;
use std::{io::{Error, ErrorKind}, cmp::Ordering};

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

#[derive(PartialEq, Eq, PartialOrd)]
struct Semester {
    pub code: String,
    pub year: String,
    pub description: String,
}

impl Ord for Semester {
    fn cmp(&self, other: &Self) -> Ordering {
        let year_int = self.year.parse::<i32>().unwrap();
        let other_year_int = other.year.parse::<i32>().unwrap();
        if year_int < other_year_int {
            Ordering::Less
        } else if year_int > other_year_int {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
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
        Number nvarchar(5) not null,
        Title nvarchar(32) not null,
        Credits nvarchar(5) not null,
        PRIMARY KEY (Prefix, Number))",
    )?;
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS Grade (
        Letter nvarchar(5) not null,
        Value nvarchar(5) not null,
        PRIMARY KEY (Letter))",
    )?;
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS Semester (
        Code nvarchar(4) not null,
        Year nvarchar(4) not null,
        Description varchar(6) not null,
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
        CourseNumber nvarchar(5) not null,
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
    // Start a transaction
    let mut transaction = conn.start_transaction(TxOpts::default())?;
    // Make a prepared statement
    let stmt = transaction.prep(
        r"INSERT INTO Course (Prefix, Number, Title, Credits)
        VALUES (?, ?, ?, ?)",
    )?;
    // Execute query in prepared statement with given variables
    transaction.exec_drop(
        stmt,
        (
            &course.prefix,
            &course.number,
            &course.title,
            &course.credits,
        ),
    )?;
    // Commit the transaction
    transaction.commit()?;
    // Send the OK! that the item was inserted
    Ok(format!(
        "{} {} {} {}",
        course.prefix, course.number, course.title, course.credits
    ))
}

fn insert_grade(conn: &mut PooledConn, grade: Grade) -> Result<String> {
    // Start a transaction
    let mut transaction = conn.start_transaction(TxOpts::default())?;
    // Make a prepared statement
    let stmt = transaction.prep(
        r"INSERT INTO Grade (Letter, Value)
        VALUES (?, ?)",
    )?;
    // Execute query in prepared statement with given variables
    transaction.exec_drop(stmt, (&grade.letter, &grade.value))?;
    // Commit the transaction
    transaction.commit()?;
    // Send the OK! that the item was inserted
    Ok(format!("{} {}", grade.letter, grade.value))
}

fn insert_semester(conn: &mut PooledConn, semester: Semester) -> Result<String> {
    // Start a transaction
    let mut transaction = conn.start_transaction(TxOpts::default())?;
    // Make a prepared statement
    let stmt = transaction.prep(
        r"INSERT INTO Semester (Code, Year, Description)
        VALUES (?, ?, ?)",
    )?;
    // Execute query in prepared statement with given variables
    transaction.exec_drop(
        stmt,
        (&semester.code, &semester.year, &semester.description),
    )?;
    // Commit the transaction
    transaction.commit()?;
    // Send the OK! that the item was inserted
    Ok(format!(
        "{} {} {}",
        semester.code, semester.year, semester.description
    ))
}

fn insert_student(conn: &mut PooledConn, student: Student) -> Result<String> {
    // Start a transaction
    let mut transaction = conn.start_transaction(TxOpts::default())?;
    // Make a prepared statement
    let stmt = transaction.prep(
        r"INSERT INTO Student (LName, FName, Phone)
        VALUES (?, ?, ?)",
    )?;
    // Execute query in prepared statement with given variables
    transaction.exec_drop(stmt, (&student.lname, &student.fname, &student.phone))?;
    // Commit the transaction
    transaction.commit()?;
    // Send the OK! that the item was inserted
    Ok(format!(
        "{} {} {}",
        student.lname, student.fname, student.phone
    ))
}

fn insert_taken_course(conn: &mut PooledConn, taken_course: TakenCourse) -> Result<String> {
    // Check if the database contains the given student
    if !search_student(conn, &taken_course)? {
        return Err(mysql::Error::IoError(Error::new(
            ErrorKind::Other,
            "Error: Unable to add item to database. Given student does not exist.\n",
        )));
    // Check if the database contains the given course
    } else if !search_course(conn, &taken_course)? {
        return Err(mysql::Error::IoError(Error::new(
            ErrorKind::Other,
            "Error: Unable to add item to database. Given course does not exist.\n",
        )));
    // Check if the database contains the given grade
    } else if !search_grade(conn, &taken_course)? {
        return Err(mysql::Error::IoError(Error::new(
            ErrorKind::Other,
            "Error: Unable to add item to database. Given grade does not exist.\n",
        )));
    // Check if the databases contains the given semester
    } else if !search_semester(conn, &taken_course)? {
        return Err(mysql::Error::IoError(Error::new(
            ErrorKind::Other,
            "Error: Unable to add item to database. Given semester does not exist.\n",
        )));
    }
    // Start a transaction
    let mut transaction = conn.start_transaction(TxOpts::default())?;
    // Make a prepared statement
    let stmt = transaction.prep(
        r"INSERT INTO TakenCourse (
            StudentLName, 
            StudentFName, 
            CoursePrefix, 
            CourseNumber, 
            GradeLetter, 
            SemesterCode)
        VALUES (?, ?, ?, ?, ?, ?)",
    )?;
    // Execute query in prepared statement with given variables
    transaction.exec_drop(
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
    // Commit the transaction
    transaction.commit()?;
    // Send the OK! that the item was inserted
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

fn search_student(conn: &mut PooledConn, taken_course: &TakenCourse) -> Result<bool> {
    // Make a prepared statement
    let stmt = conn.prep("SELECT * FROM Student WHERE Student.LName = ? AND Student.FName = ?")?;
    // Execute query in prepared statement with given variables and store results in vector
    let query = conn.exec_map(stmt,
        (&taken_course.student_lname, &taken_course.student_fname), 
        |(lname, fname, phone)|
        Student {
            lname,
            fname, 
            phone
        }
    )?;
    // Check if any results were found
    if query.is_empty() {
        return Ok(false);
    }
    Ok(true)
}

fn search_course(conn: &mut PooledConn, taken_course: &TakenCourse) -> Result<bool> {
    // Make a prepared statement
    let stmt = conn.prep("SELECT * FROM Course WHERE Course.Prefix = ? AND Course.Number = ?")?;
    // Execute query in prepared statement with given variables and store results in vector
    let query = conn.exec_map(stmt,
        (&taken_course.course_prefix, &taken_course.course_number), 
        |(prefix, number, title, credits)|
        Course {
            prefix,
            number,
            title,
            credits
        }
    )?;
    // Check if any results were found
    if query.is_empty() {
        return Ok(false);
    }
    Ok(true)
}

fn search_grade(conn: &mut PooledConn, taken_course: &TakenCourse) -> Result<bool> {
    // Make a prepared statement
    let stmt = conn.prep("SELECT * FROM Grade WHERE Grade.Letter = ?")?;
    // Execute query in prepared statement with given variables and store results in vector
    let query = conn.exec_map(stmt,
        (&taken_course.grade_letter,), 
        |(letter, value)|
        Grade {
            letter,
            value,
        }
    )?;
    // Check if any results were found
    if query.is_empty() {
        return Ok(false);
    }
    Ok(true)
}

fn search_semester(conn: &mut PooledConn, taken_course: &TakenCourse) ->Result<bool> {
    // Make a prepared statement
    let stmt = conn.prep("SELECT * FROM Semester WHERE Semester.Code = ?")?;
    // Execute query in prepared statement with given variables and store results in vector
    let query = conn.exec_map(stmt,
        (&taken_course.semester_code,), 
        |(code, year, description)|
        Semester {
            code,
            year,
            description
        }
    )?;
    // Check if any results were found
    if query.is_empty() {
        return Ok(false);
    }
    Ok(true)
}
// End insert functions

// Start delete functions
/// Function to delete items from the database. Possible items to delete are students.
pub fn delete_from_database(conn: &mut PooledConn, input: Vec<&str>) -> Result<()> {
    delete_student(
        conn,
        Student {
            // Check if user actually entered something
            lname: match input.get(2) {
                Some(item) => item.to_string(),
                None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other,
                        "Error: Unable to delete student. No student supplied.\n",
                )))
            },
            // Check if user actually entered something
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
    // Start a transaction
    let mut transaction = conn.start_transaction(TxOpts::default())?;
    // Make a prepared statement
    let stmt = transaction.prep(
        r"DELETE FROM Student
        WHERE Student.LName = ? AND Student.FName = ?",
    )?;
    // Execute query in prepared statement with given variables
    transaction.exec_drop(stmt, (&student.lname, &student.fname))?;
    // Make a prepared statement
    let stmt = transaction.prep(
        r"DELETE FROM TakenCourse
        WHERE TakenCourse.StudentLName = ? AND TakenCourse.StudentFName = ?",
    )?;
    // Execute query in prepared statement with given variables
    transaction.exec_drop(stmt, (student.lname, student.fname))?;
    // Commit the transaction
    transaction.commit()
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
    // Select all rows from table
    let query = conn.query_map(
        r"SELECT * FROM Course",
        |(prefix, number, title, credits)|
        Course {
            prefix,
            number,
            title,
            credits,
        },
    )?;
    // Print out header for table
    println!("+--------+--------+----------------------------------+---------+");
    println!("| Prefix | Number | Title                            | Credits |");
    println!("+--------+--------+----------------------------------+---------+");
    // Iterate through rows from query and print out styled table
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
    // Select all rows from table
    let query = conn.query_map(r"SELECT * FROM Grade", |(letter, value)| Grade {
        letter,
        value,
    })?;
    // Print out header for table
    println!("+------+-------+");
    println!("+ Type | Value |");
    println!("+------+-------+");
    // Iterate through rows from query and print out styled table
    for item in query.iter() {
        println!("| {:<4} | {:>5} |", item.letter, item.value);
    }
    println!("+------+-------+\n");
    Ok(())
}

fn list_semesters(conn: &mut PooledConn) -> Result<()> {
    // Select all rows from table
    let query = conn.query_map(r"SELECT * FROM Semester", |(code, year, description)| {
        Semester {
            code,
            year,
            description,
        }
    })?;
    // Print out header for table
    println!("+------+------+--------+");
    println!("| Code | Year | Desc   |");
    println!("+------+------+--------+");
    // Iterate through rows from query and print out styled table
    for item in query.iter() {
        println!("| {:<4} | {:>4} | {:<6} |", item.code, item.year, item.description);
    }
    println!("+------+------+--------+\n");
    Ok(())
}

fn list_students(conn: &mut PooledConn) -> Result<()> {
    // Select all rows from table
    let query = conn.query_map(r"SELECT * FROM Student", |(lname, fname, phone)| Student {
        lname,
        fname,
        phone,
    })?;
    // Print out header for table
    println!("+-----------------+-----------------+----------------------+");
    println!("| Last Name       | First Name      | Phone Number         |");
    println!("+-----------------+-----------------+----------------------+");
    // Iterate through rows from query and print out styled table
    for item in query.iter() {
        println!("| {:<15} | {:<15} | {:>20} |", item.lname, item.fname, item.phone);
    }
    println!("+-----------------+-----------------+----------------------+\n");
    Ok(())
}

fn list_taken_courses(conn: &mut PooledConn) -> Result<()> {
    // Select all rows from table
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
    // Print out header for table
    println!("+-----------------+-----------------+--------+--------+------+------+");
    println!("| Last Name       | First Name      | Prefix | Number | Type | Code |");
    println!("+-----------------+-----------------+--------+--------+------+------+");
    // Iterate through rows from query and print out styled table
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

pub fn transcript(conn: &mut PooledConn, input: Vec<&str>) -> Result<()> {
    print_transcript(conn, 
        Student {
            // Check if user actually entered something
            lname: match input.get(1) {
                Some(item) => item.to_string(),
                None => return Err(mysql::Error::IoError(Error::new(
                        ErrorKind::Other,
                        "Error: Unable to print transcript. No student supplied.\n",
                    )))
            },
            // Check if user actually entered something
            fname: match input.get(2) {
                Some(item) => item.to_string(),
                None => return Err(mysql::Error::IoError(Error::new(
                    ErrorKind::Other, 
                    "Error: Unable to print transcript. Student's first name not supplied.\n"
                    )))
            },
            phone: String::new(),
        },
    )?;
    Ok(())
}

fn print_transcript(conn: &mut PooledConn, student: Student) -> Result<()> {
    let stmt = conn.prep(
        r"SELECT Semester.Description, Semester.Year, Course.Prefix, Course.Number, 
        Course.Title, Course.Credits, Grade.Letter, Grade.Value
        FROM TakenCourse, Semester, Course, Grade
        WHERE TakenCourse.StudentLName = ? AND TakenCourse.StudentFName = ? 
        AND TakenCourse.CoursePrefix = Course.Prefix AND TakenCourse.CourseNumber = Course.Number 
        AND TakenCourse.GradeLetter = Grade.Letter AND TakenCourse.SemesterCode = Semester.Code
        GROUP BY Semester.Code"
    )?;
    let taken_courses = conn.exec_map(stmt, 
        (&student.lname, &student.fname), 
        |(student_lname, student_fname, course_prefix, course_number, grade_letter, semester_code)| 
        TakenCourse {
            student_lname,
            student_fname,
            course_prefix,
            course_number,
            grade_letter,
            semester_code,
        }
    )?;
    if taken_courses.is_empty() {
        println!("Error: Unable to print transcript. Given student either does not exist or has not taken and courses yet.\n");
        return Ok(());
    }
    let mut courses: Vec<Course> = Vec::new();
    let mut grades: Vec<Grade> = Vec::new();
    let mut semesters: Vec<Semester> = Vec::new();
    for taken_course in taken_courses.iter() {
        courses.append(&mut conn.exec_map(r"SELECT * FROM Course WHERE Course.Prefix = ? AND Course.Number = ?", 
            (&taken_course.course_prefix, &taken_course.course_number), 
            |(prefix, number, title, credits)| Course {prefix, number, title, credits})?);
        grades.append(&mut conn.exec_map(r"SELECT * FROM Grade WHERE Grade.Letter = ?", 
            (&taken_course.grade_letter,), 
            |(letter, value)| Grade {letter, value})?);
        semesters.append(&mut conn.exec_map(r"SELECT * FROM Semester WHERE Semester.Code = ?", 
        (&taken_course.semester_code,), 
            |(code, year, description)| Semester {code, year, description})?);
    }
    semesters.sort();
    let mut sum_credits = 0;
    let mut sum_grade_value = 0.0;
    let num_courses = taken_courses.len() as f32;
    for semester in semesters.iter() {
        println!("============ Semester: {:<6} {} ============", semester.description, semester.year);
        for taken_course in taken_courses.iter() {
            if taken_course.semester_code == semester.code {
                for course in courses.iter() {
                    if course.prefix == taken_course.course_prefix && course.number == taken_course.course_number {
                        for grade in grades.iter() {
                            if grade.letter == taken_course.grade_letter {
                                sum_credits = sum_credits + course.credits.parse::<i32>().unwrap();
                                sum_grade_value = sum_grade_value + grade.value.parse::<f32>().unwrap();
                                println!("{}{} {} ({}) {}", course.prefix, course.number, course.title, course.credits, taken_course.grade_letter);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("  STUDENT HOURS COMPLETED: {sum_credits}");
    println!("  STUDENT GPA: {}\n", sum_grade_value/num_courses);
    Ok(())
}