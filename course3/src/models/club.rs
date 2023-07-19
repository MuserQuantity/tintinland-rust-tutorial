// club.rs

use std::collections::HashSet;

pub struct Club {
    id: u32,
    name: String,
    members: HashSet<u32>, // 使用 HashSet 存成员 id
}

impl Club {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            members: HashSet::new(),
        }
    }

    pub fn add_member(&mut self, member_id: u32) {
        self.members.insert(member_id);
    }

    pub fn remove_member(&mut self, member_id: u32) {
        self.members.remove(&member_id);
    }

    pub fn update(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_members(&self) -> &HashSet<u32> {
        &self.members
    }
}
