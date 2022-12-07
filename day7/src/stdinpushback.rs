use std::io::stdin;

pub struct Pushback {
    pub value: Option<String>,
}

impl Pushback {
    pub fn get(&mut self, buf: &mut String) -> usize {
        match &self.value {
            None => {stdin().read_line(buf).unwrap()}
            Some(v) => {*buf = v.to_string(); self.value = None; (*buf).len() }
        }
    }
    pub fn set(&mut self, value: String) {
        self.value = Some(value);
    }
}