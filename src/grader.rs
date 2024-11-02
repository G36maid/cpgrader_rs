// grader.rs
use crate::Student;
use crate::unzip_student_file; // Adjust the path if necessary
use crate::run_make; // Adjust the path if necessary
//use crate::log_errors; // Adjust the path if necessary
use std::io;
use std::str;
use std::io::Write;
//use colored_diff::diff;
//use crate::cleanup_student_folder; // Add this line to import the function
use std::process::Command;
//use std::io::Write;
use std::fs;


pub fn grade_student(student: &mut Student,homework_name: &str ,problem_name: &str, testcase_name: &str)-> Result<(), Box<dyn std::error::Error>> {
    // Iterate over all test cases

    let config_content = fs::read_to_string("./config.toml")?;
    let config: toml::Value = toml::from_str(&config_content)?;


    let testcase_num = config
        .get("testcase03")
        .and_then(|section| section.get("testcase"))
        .and_then(|value| value.as_integer())
        .unwrap_or(0);
    println!("{}", testcase_name);
    println!("testcase_num: {}", testcase_num);
    let mut total_score: i32 = 0;
    for i in 1..=testcase_num { // Assuming there are 10 test cases
        println!("running testcase: {}", i);
        //todo run testcase
        //input testcase score
        
        // ./grader/studentID/studentID_HW01/hw010X < "./testcase/testcase05/in/${i}.in"
        let student_output_folder = format!("./grader/{}/{}_{}/out", student.id, student.id , homework_name);
        
        let testcase_command = format!("./grader/{}/{}_{}/{} < ./testcase/{}/in/{}.in > {}/{}.out " , student.id, student.id , homework_name, problem_name, testcase_name, i ,  student_output_folder, i);
        
        println!("testcase_command: {}", testcase_command);
        
        //format!("> {}/{}.out", student_output_folder, i)
        //println!("student_output_folder: {}", student_output_folder);
        
        fs::create_dir_all(&student_output_folder)?;

        let output = Command::new("sh")
            .arg("-c")
            .arg(&testcase_command)
            //.arg()
            .output()?;
        
        //println!("output: {:?}", output);

        if output.status.success() {
            let stdout = std::str::from_utf8(&output.stdout).unwrap();
            //println!("Command succeeded with output: {}", stdout);
            let expected_output = std::fs::read_to_string(format!("./testcase/{}/out/{}.out", testcase_name, i))?;
            //println!("expected_output: {}", expected_output);

            let diff_command = format!("colordiff -s -y -Z -b {}/{}.out ./testcase/{}/out/{}.out", student_output_folder, i, testcase_name, i);
            println!("diff_command: {}", diff_command);
            let diff_output = Command::new("sh")
                .arg("-c")
                .arg(&diff_command)
                .spawn()?
                .wait_with_output()?;
            io::stdout().write_all(&diff_output.stdout)?;
        } else {
            let stderr = std::str::from_utf8(&output.stderr).unwrap();
            eprintln!("Command failed with error: {}", stderr);
        }
        println!("Please enter the score for test case {}:", i);

        let mut score = String::new();
        io::stdin().read_line(&mut score)?;
        let score: i32 = score.trim().parse()?;
        total_score += score;
    
    }

    //    Print the total score
    println!("total_score：{}", total_score);

    // Update the student's score
    student.grades.insert(problem_name.to_string(), total_score.to_string());
    student.is_graded = true;
    println!("updated student score: {:?}", student.grades);
    println!("please clean up the student folder by running `cargo run -- clean`");
    Ok(())
}

pub fn grader(student: &mut Student, homework_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if student.is_graded {
        println!("學生 {} 已經被評分過。", student.id);
        //return Ok(());
    }
    println!("處理學生：{} - {}", student.index, student.name);
    if let Some(zip_file) = &student.zip_file {
        if let Err(e) = unzip_student_file(student, "./grader") {
            student.errors.push(format!("解壓縮錯誤: {}", e));
        } else if let Err(e) = run_make(student, homework_name) {
            student.errors.push(format!("make 錯誤: {}", e));
        }
    } else {
        student.errors.push(format!("學生 {} 沒有 zip 檔案。", student.id));
    }

    // if !student.errors.is_empty() {
    //     //log_errors(student)?;
    // }

    Ok(())
}
pub fn prompt_for_grade(student: &mut Student, homework_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("請輸入學生 {} 的 {} 成績：", student.id, homework_name);
    let mut grade = String::new();
    io::stdin().read_line(&mut grade)?;
    let grade = grade.trim().to_string();
    student.grades.insert(homework_name.to_string(), grade.clone());
    println!("學生 {} 的 {} 成績為：{}", student.id, homework_name, grade);
    Ok(())
}
pub fn score_student(student: &mut Student, problem: &str, score: &str) -> Result<(), Box<dyn std::error::Error>> {
    student.grades.insert(problem.to_string(), score.to_string());
    student.is_graded = true;
    Ok(())
}