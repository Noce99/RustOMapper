mod punctual;
mod linear;
mod area;

pub trait Symbol{
    fn render(&self);
}

pub struct SymbolsBag {
    bag: Vec<Box<dyn Symbol>>
}

impl SymbolsBag {
    pub fn new() -> Self {
        Self { bag: Vec::new() }
    }
    pub fn insert(&mut self, symbol: Box<dyn Symbol>) {
        self.bag.push(symbol);
    }
    pub fn len(&self) -> usize {
        self.bag.len()
    }
}