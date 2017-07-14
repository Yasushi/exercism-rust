use std::collections::BTreeMap;

pub struct School {
    roster: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School { roster: BTreeMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster.entry(grade).or_insert(Vec::new()).push(
            student.to_string(),
        );
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().cloned().collect()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        let mut x = self.roster.get(&grade).cloned();
        for v in x.as_mut() {
            v.sort();
        }
        x
    }
}
