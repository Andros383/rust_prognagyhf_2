#[derive(Debug)]
pub struct Wire {
    front: i32,
    back: i32,
}

impl Wire {
    pub fn new() -> Self {
        Self { front: 0, back: 0 }
    }
    pub fn read(&self) -> i32 {
        self.back
    }
    pub fn write(&mut self, new_value: i32) {
        self.front = self.front.max(new_value);
    }
    pub fn update(&mut self) {
        self.back = self.front;
        self.front = 0;
    }
    // TODO nem fog wire-t kiírni
}
