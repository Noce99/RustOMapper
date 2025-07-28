use std::rc::Rc;
use crate::map::symbols::{Symbol, SymbolCommon};
use crate::map_file::reading::Node;

pub struct LinearSymbol{
    pub id: i32,
    pub code: String,
    pub name: String,
    pub description: String,
}

impl LinearSymbol{
    pub fn symbol_from_a_node(basic_symbol: &SymbolCommon, node: &Rc<Node>) -> Option<Box<Self>> {
        Some(Box::new(LinearSymbol{
            id: basic_symbol.id.clone(),
            code: basic_symbol.code.clone(),
            name: basic_symbol.name.clone(),
            description: basic_symbol.description.clone()
        }))
    }
}

impl Symbol for LinearSymbol {
    fn render(&self) {
        todo!()
    }

    fn show(&self) -> String {
        format!("{} [Linear Symbol] ({})", self.name, self.id)
    }

    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_symbol_type(&self) -> String {
        "linear".to_string()
    }
}