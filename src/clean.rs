//use crate::Student;
use std::fs;

pub fn cleanup_student_folder() -> Result<(), Box<dyn std::error::Error>> {
    let student_output_dir = format!("./grader/");
    if fs::read_dir(&student_output_dir)?.next().is_none() {
        return Ok(());
    }
    for entry in fs::read_dir(&student_output_dir)? {
        let entry = entry?;
        if entry.path().is_dir() {
            fs::remove_dir_all(entry.path())?;
        } else {
            fs::remove_file(entry.path())?;
        }
    }
    println!("successfully cleaned up student folder");
    Ok(())
}
