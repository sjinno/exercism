use std::collections::HashMap;

#[allow(clippy::new_without_default)]
#[derive(Clone, Debug)]
pub struct School {
    students: HashMap<u32, Vec<String>>,
}

impl Default for School {
    fn default() -> Self {
        Self::new()
    }
}

impl School {
    pub fn new() -> School {
        Self {
            students: Default::default(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if let Some(s) = self.students.get_mut(&grade) {
            s.push(student.to_string());
        } else {
            self.students
                .entry(grade)
                .or_insert_with(|| vec![student.to_string()]);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self.students.keys().copied().collect::<Vec<u32>>();
        grades.sort_by(|a, b| a.partial_cmp(b).unwrap());
        grades
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        if let Some(s) = self.students.get(&grade) {
            let mut students = s.to_vec();
            students.sort_by_key(|a| a.to_lowercase());
            students
        } else {
            Vec::<String>::new()
        }
    }
}
