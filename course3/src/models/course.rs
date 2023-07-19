// course.rs

use std::collections::HashMap;

pub struct Course {
    id: u32,
    name: String,
    classes: HashMap<u32, String>, // 以class id为键,class name为值
}

impl Course {
    
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            classes: HashMap::new(), 
        }
    }

    pub fn add_class(&mut self, class_id: u32, class_name: &str) {
        self.classes.insert(class_id, class_name.to_string());
    }

    pub fn remove_class(&mut self, class_id: u32) {
        self.classes.remove(&class_id);
    }
    
    pub fn update_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
}