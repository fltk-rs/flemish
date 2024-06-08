use chrono::{offset::Local, NaiveDate};

pub struct Model {
    pub direct: i32,
    pub start: String,
    pub back: String,
    pub start_active: bool,
    pub back_active: bool,
    pub book_active: bool,
}

impl Model {
    pub fn direct(&mut self, value: i32) {
        self.direct = value;
        self.refresh();
    }
    pub fn start(&mut self, value: String) {
        self.start = value;
        self.refresh();
    }
    pub fn back(&mut self, value: String) {
        self.back = value;
        self.refresh();
    }
    pub fn default() -> Self {
        let current = Local::now()
            .naive_local()
            .date()
            .format("%Y-%m-%d")
            .to_string();
        Self {
            direct: 0,
            start: current.clone(),
            start_active: true,
            back: current,
            back_active: false,
            book_active: false,
        }
    }
    pub fn refresh(&mut self) {
        if self.direct == 0 {
            self.back_active = false;
            self.book_active = get_date(&self.start).is_ok();
        } else {
            self.back_active = true;
            let start_date = get_date(&self.start);
            let back_date = get_date(&self.back);
            self.book_active = start_date.is_ok()
                && back_date.is_ok()
                && start_date.unwrap() <= back_date.unwrap();
        }
    }
}

fn get_date(value: &str) -> Result<NaiveDate, chrono::ParseError> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d")
}
