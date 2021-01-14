use std::collections::HashMap;

#[allow(clippy::new_without_default)]
#[derive(Clone, Debug)]
pub struct School {
    students: HashMap<u32, Vec<String>>,
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
                .or_insert(vec![student.to_string()]);
        }
        // println!("{:?}", self.students);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self
            .students
            .keys()
            .map(|grade| *grade)
            .collect::<Vec<u32>>();
        grades.sort_by(|a, b| a.partial_cmp(b).unwrap());
        grades
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        // println!("{:?}", self.students);
        if let Some(s) = self.students.get(&grade) {
            let mut students = s.iter().map(|name| name.clone()).collect::<Vec<String>>();
            students.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
            students
        } else {
            Vec::<String>::new()
        }
    }
}
