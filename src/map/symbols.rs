mod punctual;
mod linear;
mod area;

pub trait Symbol{
    fn render(&self);
}

pub struct SymbolsBag {
    // There we are creating a Vector of Box.
    // A Box is simply a container that store the data on the heap instead that on the stack, when
    // the owner change just the pointer in te stack is copied and not the real data on the heap.
    // dyn Symbol means that inside the Boxes we can have any struct that implement the Symbol trait
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