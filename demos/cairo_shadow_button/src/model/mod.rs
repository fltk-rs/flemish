use std::{env, fs};

#[derive(Debug, Clone)]
pub struct Model {
    value: u8,
}

impl Model {
    pub fn default() -> Self {
        if let Ok(value) = fs::read(file()) {
            return Self { value: value[0] };
        };
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
    pub fn save(&mut self) {
        fs::write(file(), [self.value]).unwrap();
    }
}

fn file() -> String {
    env::var("HOME").unwrap() + "/.config/" + crate::NAME
}
