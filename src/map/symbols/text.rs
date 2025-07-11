use crate::map::symbols::Symbol;

pub struct TextSymbol{
    id: u32,
    code: String,
    name: String,
    description: String,
}

impl TextSymbol{
    pub fn new(id:u32, code:String, name:String, description:String) -> Self{
        TextSymbol{
            id,
            code,
            name,
            description
        }
    }
}

impl Symbol for TextSymbol{
    fn render(&self) {
        todo!()
    }

    fn show(&self) -> String{
        format!("{} [Text Symbol] ({})", self.name, self.id)
    }
}