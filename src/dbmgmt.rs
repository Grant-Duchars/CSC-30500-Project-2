use mysql::prelude::*;
use mysql::*;

pub struct Course {
    prefix: String,
    number: i32,
    title: String,
    credits: i32,
}

pub struct Grade {
    letter: String,
    value: f32,
}

pub struct Semester {
    code: String,
    year: i32,
    description: String,
}

pub struct Student {
    lname: String,
    fname: String,
    phone: i32,
}

pub struct TakenCourse {
    student_lname: String,
    student_fname: String,
    course_prefix: String,
    course_number: i32,
    grade_letter: String,
    semester_code: String,
}

pub fn setup_database(conn: &mut PooledConn) -> Result<()> {
    conn.query_drop(
        r"CREATE TABLE Course (
        Prefix varchar(5) not null,
        Number int not null,
        Title varchar(32),
        Credits int)",
    )?;
    conn.query_drop(
        r"CREATE TABLE Grade (
        Letter varchar(5) not null,
        Value float not null)",
    )?;
    conn.query_drop(
        r"CREATE TABLE Semester (
        Code varchar(5) not null,
        Year int,
        Description varchar(10))",
    )?;
    conn.query_drop(
        r"CREATE TABLE Student (
        LName varchar(32) not null,
        FName varchar(32) not null,
        Phone int)",
    )?;
    conn.query_drop(
        r"CREATE TABLE TakenCourse (
        StudentLName varchar(32) not null,
        StudentFName varchar(32) not null,
        CoursePrefix varchar(5) not null,
        CourseNumber int not null,
        GradeLetter varchar(5) not null,
        SemesterCode varchar(5))",
    )?;
    Ok(())
}

pub fn insert_course(conn: &mut PooledConn, course: Course) -> Result<()> {
    conn.exec_drop(
        r"INSERT INTO Course (Prefix, Number, Title, Credits)
        VALUES (:Prefix, :Number, :Title, :Credits)",
        params! {
            "Prefix" => course.prefix,
            "Number" => course.number,
            "Title" => course.title,
            "Credits" => course.credits
        },
    )?;
    Ok(())
}

pub fn insert_grade(conn: &mut PooledConn, grade: Grade) -> Result<()> {
    conn.exec_drop(
        r"INSERT INTO Grade (Letter, Value)
        VALUES (:Letter, :Value)",
        params! {
            "Prefix" => grade.letter,
            "Number" => grade.value,
        },
    )?;
    Ok(())
}

pub fn insert_semester(conn: &mut PooledConn, grade: Grade) -> Result<()> {
    conn.exec_drop(
        r"INSERT INTO Grade (Letter, Value)
        VALUES (:Letter, :Value)",
        params! {
            "Prefix" => grade.letter,
            "Number" => grade.value,
        },
    )?;
    Ok(())
}
