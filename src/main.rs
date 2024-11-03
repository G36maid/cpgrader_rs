mod build;
mod clean;
mod grader;
mod load;
mod log;
mod student;
use build::build;
use clap::{command, Arg, Command};
use clean::cleanup_student_folder;
use grader::{grade_student, score_student};
use load::load;
use load::{store, to_csv};
use std::fs;
use student::Student;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = "./example"; // 替換成你的目標資料夾路徑
    let mut students = load(target_dir)?;

    let config_content = fs::read_to_string("config.toml")?;
    let config: toml::Value = toml::from_str(&config_content)?;
    let homework_name = config["global"]["homework"].as_str().unwrap().to_string();
    // for student in &students {
    //     println!("{:?}", student);
    // }

    let match_result = command!()
        .subcommand(
            Command::new("grade")
                .about("Automatically grade students by dependcies")
                .arg(
                    Arg::new("index")
                        .short('i')
                        .long("index")
                        //.required(true)
                        .help("This argument define which person to build")
                        .conflicts_with("id"),
                )
                .arg(
                    Arg::new("id")
                        //.short('id')
                        .long("id")
                        .aliases(["id", "ID"])
                        //.required(true)
                        .help("Or you can use student id"),
                )
                .arg(
                    Arg::new("problem")
                        .short('p')
                        .long("problem")
                        .required(true)
                        .help("This argument define which problem to grade"),
                )
                .arg(
                    Arg::new("testcase")
                        .short('t')
                        .long("testcase")
                        .required(true)
                        .help("This argument define which testcase to grade"),
                ),
        )
        .subcommand(
            Command::new("score")
                .about("Input students score for one problem")
                .arg(
                    Arg::new("index")
                        .short('i')
                        .long("index")
                        //.required(true)
                        .help("This argument define which person to build")
                        .conflicts_with("id"),
                )
                .arg(
                    Arg::new("id")
                        //.short('id')
                        .long("id")
                        .aliases(["id", "ID"])
                        //.required(true)
                        .help("Or you can use student id"),
                )
                .arg(
                    Arg::new("problem")
                        .short('p')
                        .long("problem")
                        .required(true)
                        .help("This argument define which problem to score"), //.conflicts_with("lastname")
                )
                .arg(
                    Arg::new("score")
                        .short('s')
                        .long("score")
                        //.aliases(["lname", "lastname"])
                        .required(true)
                        .help("The score of homework"),
                ),
        )
        .subcommand(
            Command::new("build")
                .about("Clean, unzip, make, copy depends to grader dir")
                .arg(
                    Arg::new("index")
                        .short('i')
                        .long("index")
                        //.required(true)
                        .help("This argument define which person to build")
                        .conflicts_with("id"),
                )
                .arg(
                    Arg::new("id")
                        //.short('id')
                        .long("id")
                        .aliases(["id", "ID"])
                        //.required(true)
                        .help("Or you can use student id"),
                )
                .arg(
                    Arg::new("problem")
                        .short('p')
                        .long("problem")
                        .required(true)
                        .help("This argument define which problem to score"), //.conflicts_with("lastname")
                ),
        )
        .subcommand(Command::new("clean").about("Clean the grader dir"))
        .subcommand(Command::new("log").about("Print the log"))
        .subcommand(Command::new("csv").about("Create CSV of grade"))
        .get_matches();

    if let Some(matches) = match_result.subcommand_matches("grade") {
        //println!("Grading students(in development)");
        let problem = matches.get_one::<String>("problem").unwrap();
        let testcase = matches.get_one::<String>("testcase").unwrap();

        let student = if let Some(index) = matches.get_one::<String>("index") {
            let index: usize = index.parse()?;
            if index >= students.len() {
                println!("Index out of bounds");
                return Ok(());
            }
            println!("Grading student at index {}", index);
            &mut students[index]
        } else if let Some(id) = matches.get_one::<String>("id") {
            if let Some(student) = students.iter_mut().find(|s| s.id == *id) {
                println!("Grading student with ID {}", id);
                student
            } else {
                println!("Student with ID {} not found", id);
                return Ok(());
            }
        } else {
            println!("Grading for first ungraded student");
            // Find the first student who hasn't been graded for the specified problem
            let index = students
                .iter()
                .position(|s| !s.grades.contains_key(problem))
                .unwrap();

            if index >= students.len() {
                println!("Index out of bounds");
                return Ok(());
            }
            println!("Grading student at index {}", index);
            &mut students[index]
        };
        //print student data
        println!("{:?}", student);
        build(student, &homework_name)?;
        grade_student(student, &homework_name, &problem, testcase)?;
        store(&students)?;
        Ok(())
    } else if let Some(matches) = match_result.subcommand_matches("score") {
        let problem = matches.get_one::<String>("problem").unwrap();
        let score = matches.get_one::<String>("score").unwrap();

        let student = if let Some(index) = matches.get_one::<String>("index") {
            let index: usize = index.parse()?;
            if index >= students.len() {
                println!("Index out of bounds");
                return Ok(());
            }
            println!("Scoring student at index {}", index);
            &mut students[index]
        } else if let Some(id) = matches.get_one::<String>("id") {
            if let Some(student) = students.iter_mut().find(|s| s.id == *id) {
                println!("Scoring student with ID {}", id);
                student
            } else {
                println!("Student with ID {} not found", id);
                return Ok(());
            }
        } else {
            println!("Scoring for first ungraded student");
            let index = students
                .iter()
                .position(|s| !s.grades.contains_key(problem))
                .unwrap();
            if index >= students.len() {
                println!("Index out of bounds");
                return Ok(());
            }
            println!(
                "Scoring student at index {} at {} with {}",
                index, problem, score
            );
            &mut students[index]
        };
        println!("{:?}", student);
        score_student(student, problem, score)?;
        store(&students)?;
        Ok(())
    } else if let Some(matches) = match_result.subcommand_matches("build") {
        let problem = matches.get_one::<String>("problem").unwrap();
        let student = if let Some(index) = matches.get_one::<String>("index") {
            println!("Building for index {}", index);
            let index: usize = index.parse()?;
            &mut students[index]
        } else if let Some(id) = matches.get_one::<String>("id") {
            println!("Building for ID {}", id);
            let index = students.iter().position(|s| s.id == *id).unwrap();
            &mut students[index]
        } else {
            println!("Building for first ungraded student");
            let index = students
                .iter()
                .position(|s| !s.grades.contains_key(problem))
                .unwrap();
            if index >= students.len() {
                println!("Index out of bounds");
                return Ok(());
            }
            &mut students[index]
        };
        //cleanup_student_folder()?;
        println!("{:?}", student);
        build(student, &homework_name)?;
        store(&students)?;
        Ok(())
    } else if let Some(matches) = match_result.subcommand_matches("clean") {
        println!("Cleaning the grader dir");
        cleanup_student_folder()?;
        store(&students)?;
        Ok(())
    } else if let Some(matches) = match_result.subcommand_matches("log") {
        println!("Printing the log");
        for student in &students {
            println!("{:?}", student);
        }
        store(&students)?;
        Ok(())
    } else if let Some(matches) = match_result.subcommand_matches("csv") {
        println!("Creating CSV of grade");
        to_csv(&students, "./grades.csv")?;
        Ok(())
    } else {
        Ok(())
    }
}
