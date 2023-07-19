// class.rs

use std::collections::HashMap;

pub struct Class {
  id: u32,
  name: String,
  students: HashMap<u32, String>, // 学生id到name的映射
}

impl Class {

  pub fn new(id: u32, name: &str) -> Self {
    Self {
      id,
      name: name.to_string(),
      students: HashMap::new(),
    }
  }

  pub fn add_student(&mut self, student_id: u32, name: &str) {
    self.students.insert(student_id, name.to_string()); 
  }
  
  pub fn remove_student(&mut self, student_id: &u32) {
    self.students.remove(student_id);
  }

  // 其他方法
}