use crate::Student;
use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::Path;
use std::process::Command;
use zip::read::ZipArchive;

pub fn build(student: &mut Student, homework_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing student: {} - {}", student.index, student.name);
    if let Some(_zip_file) = &student.zip_file {
        if let Err(e) = unzip_student_file(student, "./grader") {
            student.errors.push(format!("Unzip error: {}", e));
        } else if let Err(e) = dependent(student, homework_name) {
            student.errors.push(format!("File copy error: {}", e));
        } else if let Err(e) = run_make(student, homework_name) {
            student.errors.push(format!("Make error: {}", e));
        }
    } else {
        student
            .errors
            .push(format!("Student {} does not have a zip file.", student.id));
    }
    Ok(())
}
fn dependent(student: &Student, homework_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // 讀取 config.toml
    let config = std::fs::read_to_string("config.toml")?;
    let config: toml::Value = toml::from_str(&config)?;
    let student_output_dir = format!("./grader/{}/{}_{}", student.id, student.id, homework_name);

    let files = config["global"]["dependent"].as_array().unwrap();

    //println!("files: {:?}", files);

    for file in files {
        let file = file.as_str().unwrap();
        let src = format!("./dependent/{}", file);
        let dest = format!("{}/{}", student_output_dir, file);
        println!("Copying dependent {} to {}", src, dest);
        fs::copy(src, dest)?;
    }

    Ok(true)
}

fn unzip_student_file(
    student: &Student,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let zip_file_path = format!(
        "{}/{}",
        student.folder_path,
        student.zip_file.as_ref().unwrap()
    );

    let file = File::open(&zip_file_path)?;
    let mut archive = ZipArchive::new(BufReader::new(file))?;

    let student_output_dir = format!("{}/{}", output_dir, student.id);
    fs::create_dir_all(&student_output_dir)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => student_output_dir.clone() + "/" + path.to_str().unwrap(),
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = Path::new(&outpath).parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

fn run_make(student: &Student, homework_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let student_output_dir = format!("./grader/{}/{}_{}", student.id, student.id, homework_name);
    let output = Command::new("make")
        .current_dir(&student_output_dir)
        .output()?;

    // let stdout = String::from_utf8_lossy(&output.stdout);
    // let stderr = String::from_utf8_lossy(&output.stderr);

    //let compile_log = format!("stdout:\n{}\nstderr:\n{}", stdout, stderr);
    //log_compile(student, &compile_log)?;

    if output.status.success() {
        println!("Student {}'s make succeeded.", student.id);
    } else {
        println!("Student {}'s make failed.", student.id);
        return Err(Box::from("make failed"));
    }

    Ok(())
}
