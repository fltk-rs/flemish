use {
    serde::{Deserialize, Serialize},
    std::fs,
};

#[derive(Deserialize, Serialize)]
pub struct Task {
    pub status: bool,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct Model {
    pub tasks: Vec<Task>,
}

impl Model {
    pub fn check(&mut self, idx: usize) {
        self.tasks[idx].status = !self.tasks[idx].status
    }
    pub fn add(&mut self, description: String) {
        self.tasks.push(Task {
            status: false,
            description,
        })
    }
    pub fn save(&mut self, file: String) {
        fs::write(file, rmp_serde::to_vec(&self).unwrap()).unwrap();
    }
}
