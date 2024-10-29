mod student;
mod extract;
mod grader;
mod grade;
mod cleanup;
mod log;

use crate::student::Student;
use crate::extract::extract_students;
use crate::grader::grader;
use crate::grade::prompt_for_grade;
use crate::cleanup::cleanup_student_folder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = "./example"; // 替換成你的目標資料夾路徑
    let mut students = extract_students(target_dir)?;

    println!("請輸入該次作業名稱：");
    let mut homework_name = String::new();
    std::io::stdin().read_line(&mut homework_name)?;
    let homework_name = homework_name.trim().to_string();

    for student in &students {
        println!("{:?}", student);
    }

    for student in &mut students {
        grader(student, &homework_name)?;
        prompt_for_grade(student, &homework_name)?;
        cleanup_student_folder(student)?;
    }

    Ok(())
}