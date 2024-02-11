use std::collections::HashMap;

#[allow(clippy::new_without_default)]
pub struct School {
    roster: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> Self {
        Self {
            roster: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster
            .entry(grade)
            .and_modify(|students| students.push(student.to_string()))
            .or_insert(vec![student.to_string()]);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut all_grades: Vec<u32> = self.roster.keys().cloned().collect();
        all_grades.sort();
        all_grades
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut grade_roster: Vec<String> = self
            .roster
            .get(&grade)
            .clone()
            .unwrap_or(&Vec::new())
            .to_vec();
        grade_roster.sort();
        grade_roster
    }
}
