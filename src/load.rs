// load.rs
use std::fs;
//use std::intrinsics::simd::simd_or;
use crate::Student;
use regex::Regex;
//use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;
//use csv::Writer;

pub fn load(target_dir: &str) -> Result<Vec<Student>, Box<dyn std::error::Error>> {
    let status_file = format!("./status/status.json");
    if Path::new(&status_file).exists() {
        let file_content = fs::read_to_string(&status_file)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        let students: Vec<Student> = serde_json::from_str(&file_content)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        return Ok(students);
    }
    // 提取學生資料
    let students = extract_students(target_dir)?;

    // 序列化學生資料
    let serialized_students =
        serde_json::to_string(&students).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    //sort students by id

    // 將序列化的資料寫入 status 檔案
    let mut file =
        File::create(status_file).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    file.write_all(serialized_students.as_bytes())
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    Ok(students)
}
pub fn store(students: &Vec<Student>) -> Result<(), Box<dyn std::error::Error>> {
    let status_file = format!("./status/status.json");
    let serialized_students =
        serde_json::to_string(students).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let mut file =
        File::create(status_file).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    file.write_all(serialized_students.as_bytes())
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    Ok(())
}

fn extract_students(target_dir: &str) -> Result<Vec<Student>, Box<dyn std::error::Error>> {
    let re = Regex::new(r"(\d{8}[a-zA-Z])\s+(\S+)_\d+_assignsubmission_file_")?;
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
            id: caps[1].to_uppercase(),
            name: caps[2].to_string(),
            zip_file,
            folder_path: path.to_str().unwrap().to_string(),
            errors: Vec::new(),
            grades: HashMap::new(),
            //is_graded: false,
        };
        students.push(student);
    }
    // Sort students by id
    students.sort_by(|a, b| a.id.cmp(&b.id));

    // Change index by id
    for (new_index, student) in students.iter_mut().enumerate() {
        student.index = new_index;
    }

    Ok(students)
}

pub fn to_csv(
    students: &mut Vec<Student>,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = csv::Writer::from_path(output_file)?;

    // Collect all unique homework keys
    let mut homework_keys: HashSet<String> = HashSet::new();
    for student in students.iter() {
        for (homework, _) in &student.grades {
            homework_keys.insert(homework.clone());
        }
    }

    
    // Convert HashSet to Vec and sort it
    let mut homework_keys: Vec<String> = homework_keys.into_iter().collect();
    homework_keys.sort();

    // Create the header dynamically
    let mut header = vec!["Index", "ID", "Name", "Zip File", "Folder Path", "Errors"];
    header.extend(homework_keys.iter().map(|key| key.as_str()));
    //header.push("Is Graded");

    // Define the custom order
    let custom_order = vec![
        "41105045E", "41211003E", "41243208S", "41075013H", "41147012S", "41247005S", "41347001S",
        "41347002S", "41347003S", "41347004S", "41347005S", "41347006S", "41347007S", "41347008S",
        "41347009S", "41347011S", "41347013S", "41347014S", "41347015S", "41347016S", "41347017S",
        "41347018S", "41347019S", "41347020S", "41347021S", "41347022S", "41347023S", "41347024S",
        "41347025S", "41347026S", "41347027S", "41347028S", "41347029S", "41347030S", "41347031S",
        "41347032S", "41347033S", "41347034S", "41347035S", "41347036S", "41347037S", "41347038S",
        "41347039S", "41347040S", "41347042S", "41347043S", "41347044S", "41347047S", "41347048S",
        "41347049S", "41347050S", "41347051S", "41347052S", "41347053S", "41347054S", "41347055S",
        "41347056S", "41347057S", "41347058S", "41347059S", "41347060S", "41347061S", "41347062S",
        "41347063S", "41347064S", "41347065S", "41347066S", "41347903S", "41271223H", "41371230H",
    ];

    // Write the header
    wtr.write_record(&header)?;

    // Clone students to avoid borrow of moved value
    let students_clone = students.clone();

    // Write student data
    // for student in students {
    //     let mut record = vec![
    //         student.index.to_string(),
    //         student.id.clone(),
    //         student.name.clone(),
    //         student.zip_file.clone().unwrap_or_default(),
    //         student.folder_path.clone(),
    //         format!("{:?}", student.errors),
    //     ];

    //     // Add grades in the order of the header
    //     for key in &homework_keys {
    //         if let Some(point) = student.grades.get(key) {
    //             record.push(point.clone());
    //         } else {
    //             record.push(String::new());
    //         }
    //     }

    //     //record.push(student.is_graded.to_string());

    //     wtr.write_record(&record)?;
    // }

    // Write student data according to custom order
    for id in &custom_order {
        if let Some(student) = students_clone.iter().find(|s| &s.id == id) {
            let mut record = vec![
                student.index.to_string(),
                student.id.clone(),
                student.name.clone(),
                student.zip_file.clone().unwrap_or_default(),
                student.folder_path.clone(),
                format!("{:?}", student.errors),
            ];

            // Add grades in the order of the header
            for key in &homework_keys {
                if let Some(point) = student.grades.get(key) {
                    record.push(point.clone());
                } else {
                    record.push(String::new());
                }
            }

            //record.push(student.is_graded.to_string());

            wtr.write_record(&record)?;
        } else {
            // If no matching student, write a line with only the ID
            let mut record = vec![String::new(); header.len()];
            record[1] = id.to_string(); // Set the ID field
            wtr.write_record(&record)?;
        }
    }

    wtr.flush()?;
    Ok(())
}
