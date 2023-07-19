// student.rs

pub struct Student {
    id: u32,
    name: String,
    age: u8,
    class_id: u32,
}

impl Student {

    pub fn new(id: u32, name: &str, age: u8, class_id: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            age,
            class_id,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_class(&mut self, new_class_id: u32) {
        self.class_id = new_class_id;
    }
    
    // 其他设置属性的方法
}