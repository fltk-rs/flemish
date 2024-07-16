pub struct Model {
    value: u8,
}

impl Model {
    pub fn default() -> Self {
        Self { value: 0u8 }
    }
    pub fn inc(&mut self) {
        self.value = self.value.saturating_add(1);
    }
    pub fn dec(&mut self) {
        self.value = self.value.saturating_sub(1);
    }
    pub fn value(&self) -> String {
        self.value.to_string()
    }
}
