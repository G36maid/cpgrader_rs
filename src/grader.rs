// grader.rs
 // Adjust the path if necessary
use crate::Student; // Adjust the path if necessary
                    //use crate::log_errors; // Adjust the path if necessary
use std::io;
use std::io::Write;
use std::str;
//use colored_diff::diff;
//use crate::cleanup_student_folder; // Add this line to import the function
use std::process::Command;
//use std::io::Write;
use std::fs;

pub fn grade_student(
    student: &mut Student,
    homework_name: &str,
    problem_name: &str,
    testcase_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Iterate over all test cases

    let config_content = fs::read_to_string("./config.toml")?;
    let config: toml::Value = toml::from_str(&config_content)?;

    let testcase_num = config
        .get("testcase03")
        .and_then(|section| section.get("testcase"))
        .and_then(|value| value.as_integer())
        .unwrap_or(0);
    println!("testcase: {} , num: {}", testcase_name, testcase_num);
    let mut total_score: f32 = 0.0;
    for i in 1..=testcase_num {
        // Assuming there are 10 test cases
        println!("running testcase: {}", i);
        //todo run testcase
        //input testcase score

        // ./grader/studentID/studentID_HW01/hw010X < "./testcase/testcase05/in/${i}.in"
        let student_output_folder = format!(
            "./grader/{}/{}_{}/out",
            student.id, student.id, homework_name
        );

        let testcase_command = format!(
            "./grader/{}/{}_{}/{} < ./testcase/{}/in/{}.in > {}/{}.out ",
            student.id,
            student.id,
            homework_name,
            problem_name,
            testcase_name,
            i,
            student_output_folder,
            i
        );

        //println!("testcase_command: {}", testcase_command);

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
            // let stdout = std::str::from_utf8(&output.stdout).unwrap();
            // //println!("Command succeeded with output: {}", stdout);
            // let expected_output =
            //     std::fs::read_to_string(format!("./testcase/{}/out/{}.out", testcase_name, i))?;
            // //println!("expected_output: {}", expected_output);
            println!("{}/{}.out", student_output_folder, i);
            println!("./testcase/{}/out/{}.out", testcase_name, i);

            let diff_command = format!(
                "colordiff -s -Z -b {}/{}.out ./testcase/{}/out/{}.out",
                student_output_folder, i, testcase_name, i
            );
            //println!("diff_command: {}", diff_command);
            let diff_output = Command::new("sh")
                .arg("-c")
                .arg(&diff_command)
                .spawn()?
                .wait_with_output()?;
            io::stdout().write_all(&diff_output.stdout)?;
            println!();
        } else {
            let stderr = std::str::from_utf8(&output.stderr).unwrap();
            eprintln!("Command failed with error: {}", stderr);
        }
        println!("Please enter the score for test case {}:", i);

        let mut score = String::new();
        io::stdin().read_line(&mut score)?;
        let score: f32 = score.trim().parse()?;
        total_score += score;
    }

    //    Print the total score
    println!("current total_score：{}", total_score);

    println!("Does the score correct? (Y/n)");
    let mut correct = String::new();
    io::stdin().read_line(&mut correct)?;
    let correct = correct.trim();
    if correct == "n" {
        println!("Please enter the correct score: (now: {})", total_score);
        let mut score = String::new();
        io::stdin().read_line(&mut score)?;
        let score: f32 = score.trim().parse()?;
        total_score = score;
    }
    // Update the student's score
    student
        .grades
        .insert(problem_name.to_string(), total_score.to_string());
    //student.is_graded = true;
    println!("updated student score: {:?}", student.grades);
    //println!("please clean up the student folder by running `cargo run -- clean`");
    Ok(())
}

pub fn score_student(
    student: &mut Student,
    problem: &str,
    score: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    student
        .grades
        .insert(problem.to_string(), score.to_string());
    //student.is_graded = true;
    Ok(())
}
