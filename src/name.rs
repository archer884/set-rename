pub struct NameGenerator<'base> {
    base: &'base str,
    padding: usize,
}

pub struct NameIter<'gen> {
    current: usize,
    generator: &'gen NameGenerator<'gen>,
}

impl<'base> NameGenerator<'base> {
    pub fn new(base: &'base str, padding: usize) -> Self {
        Self {
            base,
            padding,
        }
    }

    pub fn names(&'base self) -> NameIter<'base> {
        NameIter {
            current: 0,
            generator: self,
        }
    }

    fn next_padded(&self, count: usize) -> String {
        format!("{}{:0padding$}", self.base, count, padding = self.padding)
    }
}

impl<'base> Iterator for NameIter<'base> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        Some(self.generator.next_padded(self.current))
    }
}
