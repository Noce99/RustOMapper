use std::rc::Rc;
use crate::map::symbols::{Symbol, SymbolCommon};
use crate::map_file::reading::Node;
use crate::map::symbols::geometric_shape::GeometricShape;


pub struct AreaSymbol{
    pub id: i32,
    pub code: String,
    pub name: String,
    pub description: String,
    pub geometric_shapes: Vec<Rc<dyn GeometricShape>>,
}

impl AreaSymbol{
    pub fn symbol_from_a_node(basic_symbol: &SymbolCommon, _node: &Rc<Node>) -> Option<Box<Self>> {
        let geometric_shapes : Vec<Rc<dyn GeometricShape>> = Vec::new();
        Some(Box::new(AreaSymbol{
            id: basic_symbol.id.clone(),
            code: basic_symbol.code.clone(),
            name: basic_symbol.name.clone(),
            description: basic_symbol.description.clone(),
            geometric_shapes: geometric_shapes,
        }))
    }
}

impl Symbol for AreaSymbol{
    // fn render(&self) {
    //     todo!()
    // }

    // fn show(&self) -> String{
    //     format!("{} [Area Symbol] ({})", self.name, self.id)
    // }

    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_symbol_type(&self) -> String {
        "area".to_string()
    }

    fn get_geometric_shapes(&self) -> & Vec<Rc<dyn GeometricShape>>{
        & self.geometric_shapes
    }

    // fn get_name(&self) -> &str{
    //     & self.name
    // }
}