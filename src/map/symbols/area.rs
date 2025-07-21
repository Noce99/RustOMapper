use std::rc::Rc;
use crate::map::symbols::{Symbol, SymbolCommon};
use crate::map::symbols::linear::LinearSymbol;
use crate::map_file::reading::Node;

pub struct AreaSymbol{
    id: i32,
    code: String,
    name: String,
    description: String,
}

impl AreaSymbol{
    pub fn symbol_from_a_node(basic_symbol: &SymbolCommon, node: &Rc<Node>) -> Option<Box<Self>> {
        Some(Box::new(AreaSymbol{
            id: basic_symbol.id.clone(),
            code: basic_symbol.code.clone(),
            name: basic_symbol.name.clone(),
            description: basic_symbol.description.clone()
        }))
    }
}

impl Symbol for AreaSymbol{
    fn render(&self) {
        todo!()
    }

    fn show(&self) -> String{
        format!("{} [Area Symbol] ({})", self.name, self.id)
    }

    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_symbol_type(&self) -> String {
        "area".to_string()
    }
}