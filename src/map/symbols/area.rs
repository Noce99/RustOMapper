use crate::map::symbols::Symbol;

pub struct AreaSymbol{
    id: u32,
    code: String,
    name: String,
    description: String,
}

impl AreaSymbol{
    pub fn new(id:u32, code:String, name:String, description:String) -> Self{
        AreaSymbol{
            id,
            code,
            name,
            description
        }
    }
}

impl Symbol for AreaSymbol{
    fn render(&self) {
        todo!()
    }

    fn show(&self) -> String{
        format!("{} [Area Symbol] ({})", self.name, self.id)
    }
}