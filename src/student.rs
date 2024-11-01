// student.rs
use std::collections::HashMap;

#[derive(Debug)]
pub struct Student {
    pub index: usize,
    pub id: String,
    pub name: String,
    pub zip_file: Option<String>,
    pub folder_path: String,
    pub errors: Vec<String>,
    pub grades: HashMap<String, String>,
    pub is_graded: bool,
}
