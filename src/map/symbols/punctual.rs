use crate::map::symbols::Symbol;

pub struct PunctualSymbol{
    id: u32,
    code: String,
    name: String,
    description: String,
}

impl PunctualSymbol{
    pub fn new(id:u32, code:String, name:String, description:String) -> Self{
        PunctualSymbol{
            id,
            code,
            name,
            description
        }
    }
}

impl Symbol for PunctualSymbol {
    fn render(&self) {
        todo!()
    }

    fn show(&self) -> String {
        format!("{} [Punctual Symbol] ({})", self.name, self.id)
    }
}