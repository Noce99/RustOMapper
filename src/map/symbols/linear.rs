use crate::map::symbols::Symbol;

pub struct LinearSymbol{
    id: u32,
    code: String,
    name: String,
    description: String,
}

impl LinearSymbol{
    pub fn new(id:u32, code:String, name:String, description:String) -> Self{
        LinearSymbol{
            id,
            code,
            name,
            description
        }
    }
}

impl Symbol for LinearSymbol {
    fn render(&self) {
        todo!()
    }

    fn show(&self) -> String {
        format!("{} [Linear Symbol] ({})", self.name, self.id)
    }
}