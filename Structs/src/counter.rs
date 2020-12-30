pub struct Counter {
    pub counter: u8,
}

impl Counter {
    pub fn new() -> Self {
        Self { counter: 0 } //return
    }

    pub fn increment(&mut self) -> u8 {
        self.counter = self.counter + 1;
        self.counter // return
    }
}
