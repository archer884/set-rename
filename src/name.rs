struct NameGenerator {
    base: String,
    count: usize,
}

impl NameGenerator {
    pub fn new(base: String) -> Self {
        Self { base, count: 0 }
    }

    pub fn next(&mut self) -> String {
        self.count += 1;
        format!("{}{}", self.base, self.count)
    }

    pub fn next_padded(&mut self, padding: usize) -> String {
        self.count += 1;
        format!("{}{:0padding$}", self.base, self.count, padding = padding)
    }
}
