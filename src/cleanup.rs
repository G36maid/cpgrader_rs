
use crate::student::Student;
use std::fs;
use std::error::Error;

pub fn cleanup_student_folder(student: &Student) -> Result<(), Box<dyn Error>> {
    let student_output_dir = format!("./grader/{}", student.id);
    fs::remove_dir_all(&student_output_dir)?;
    println!("已刪除學生 {} 的資料夾。", student.id);
    Ok(())
}