//use crate::Student;
use std::fs;

pub fn cleanup_student_folder() -> Result<(), Box<dyn std::error::Error>> {
    let student_output_dir = format!("./grader/");
    if fs::read_dir(&student_output_dir)?.next().is_none() {
        return Ok(());
    }
    fs::remove_dir_all(&student_output_dir)?;
    println!("successfully cleanup student folder");
    Ok(())
}
