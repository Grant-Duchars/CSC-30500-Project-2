use mysql::prelude::*;
use mysql::*;

pub struct Course<'a> {
    pub prefix: &'a str,
    pub number: &'a str,
    pub title: &'a str,
    pub credits: &'a str,
}

pub struct Grade<'a> {
    pub letter: &'a str,
    pub value: &'a str,
}

pub struct Semester<'a> {
    pub code: &'a str,
    pub year: &'a str,
    pub description: &'a str,
}

pub struct Student<'a> {
    pub lname: &'a str,
    pub fname: &'a str,
    pub phone: &'a str,
}

pub struct TakenCourse<'a> {
    pub student_lname: &'a str,
    pub student_fname: &'a str,
    pub course_prefix: &'a str,
    pub course_number: &'a str,
    pub grade_letter: &'a str,
    pub semester_code: &'a str,
}

pub fn setup_database(conn: &mut PooledConn) -> Result<()> {
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS Course (
        Prefix varchar(5) not null,
        Number int not null,
        Title varchar(32) not null,
        Credits int not null)",
    )?;
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS Grade (
        Letter varchar(5) not null,
        Value float not null)",
    )?;
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS Semester (
        Code varchar(5) not null,
        Year int not null,
        Description varchar(10) not null)",
    )?;
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS Student (
        LName varchar(32) not null,
        FName varchar(32) not null,
        Phone int not null)",
    )?;
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS TakenCourse (
        StudentLName varchar(32) not null,
        StudentFName varchar(32) not null,
        CoursePrefix varchar(5) not null,
        CourseNumber int not null,
        GradeLetter varchar(5) not null,
        SemesterCode varchar(5) not null)",
    )?;
    Ok(())
}

// Start insert functions
pub fn insert_course(conn: &mut PooledConn, course: Course) -> Result<()> {
    let stmt = conn.prep(
        r"INSERT INTO Course (Prefix, Number, Title, Credits)
        VALUES (?, ?, ?, ?)",
    )?;
    conn.exec_drop(
        stmt,
        (course.prefix, course.number, course.title, course.credits),
    )?;
    Ok(())
}

pub fn insert_grade(conn: &mut PooledConn, grade: Grade) -> Result<()> {
    let stmt = conn.prep(
        r"INSERT INTO Grade (Letter, Value)
        VALUES (?, ?)",
    )?;
    conn.exec_drop(stmt, (grade.letter, grade.value))?;
    Ok(())
}

pub fn insert_semester(conn: &mut PooledConn, semester: Semester) -> Result<()> {
    let stmt = conn.prep(
        r"INSERT INTO Semester (Code, Year, Description)
        VALUES (?, ?, ?)",
    )?;
    conn.exec_drop(stmt, (semester.code, semester.year, semester.description))?;
    Ok(())
}

pub fn insert_student(conn: &mut PooledConn, student: Student) -> Result<()> {
    let stmt = conn.prep(
        r"INSERT INTO Student (LName, FName, Phone)
        VALUES (?, ?, ?)",
    )?;
    conn.exec_drop(stmt, (student.lname, student.fname, student.phone))?;
    Ok(())
}

pub fn insert_taken_course(conn: &mut PooledConn, taken_course: TakenCourse) -> Result<()> {
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
            taken_course.student_lname,
            taken_course.student_fname,
            taken_course.course_prefix,
            taken_course.course_number,
            taken_course.grade_letter,
            taken_course.semester_code,
        ),
    )?;
    Ok(())
}
// End insert functions
pub fn delete_course(conn: &mut PooledConn, course: Course) -> Result<()> {
    let stmt = conn.prep(
        r"DELETE FROM Course
        WHERE Course.Prefix = ? AND Course.Number = ?",
    )?;
    conn.exec_drop(stmt, (course.prefix, course.number))?;
    Ok(())
}

pub fn delete_grade(conn: &mut PooledConn, grade: Grade) -> Result<()> {
    let stmt = conn.prep(
        r"DELETE FROM Grade
        WHERE Grade.Letter = ? AND Grade.Value = ?",
    )?;
    conn.exec_drop(stmt, (grade.letter, grade.value))?;
    Ok(())
}

pub fn delete_semester(conn: &mut PooledConn, semester: Semester) -> Result<()> {
    let stmt = conn.prep(
        r"DELETE FROM Semester
        WHERE Semester.Code = ?",
    )?;
    conn.exec_drop(stmt, (semester.code,))?;
    Ok(())
}
// Start delete functions

// End delete functions

// Start list functions

// End list functions
