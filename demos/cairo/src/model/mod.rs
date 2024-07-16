pub struct Model {
    pub state: [bool; 3],
}

impl Model {
    pub fn default() -> Self {
        Self { state: [true; 3] }
    }
    pub fn change(&mut self, idx: usize) {
        self.state[idx] = !self.state[idx];
    }
}
