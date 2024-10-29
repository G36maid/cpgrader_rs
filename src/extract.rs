use crate::student::Student;
use regex::Regex;
use std::fs;
use std::error::Error;
use std::collections::HashMap;

pub fn extract_students(target_dir: &str) -> Result<Vec<Student>, Box<dyn Error>> {
    let re = Regex::new(r"(\d{8}[A-Z])\s+(\S+)_\d+_assignsubmission_file_")?;
    let mut students = Vec::new();
    let entries = fs::read_dir(target_dir)?;

    for (index, entry) in entries.enumerate() {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let folder_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => continue,
        };

        let caps = match re.captures(folder_name) {
            Some(caps) => caps,
            None => continue,
        };

        let mut zip_file = None;
        let student_entries = fs::read_dir(&path)?;
        for student_entry in student_entries {
            let student_entry = student_entry?;
            let student_path = student_entry.path();
            if student_path.extension().and_then(|ext| ext.to_str()) == Some("zip") {
                if let Some(file_name) = student_path.file_name().and_then(|n| n.to_str()) {
                    zip_file = Some(file_name.to_string());
                    break;
                }
            }
        }

        let student = Student {
            index,
            id: caps[1].to_string(),
            name: caps[2].to_string(),
            zip_file,
            folder_path: path.to_str().unwrap().to_string(),
            errors: Vec::new(),
            grades: HashMap::new(),
        };
        students.push(student);
    }

    Ok(students)
}