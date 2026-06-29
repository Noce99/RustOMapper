use std::rc::Rc;
use crate::map::symbols::{Symbol, SymbolCommon};
use crate::map_file::reading::Node;
use crate::map::symbols::geometric_shape::{GeometricShape, Area};


pub struct AreaSymbol{
    pub id: i32,
    pub code: String,
    pub name: String,
    pub description: String,
    pub geometric_shapes: Vec<Rc<dyn GeometricShape>>,
    pub inner_color: Option<u32>,
}

impl AreaSymbol{
    pub fn symbol_from_a_node(basic_symbol: &SymbolCommon, node: &Rc<Node>) -> Option<Box<Self>> {
        let mut geometric_shapes : Vec<Rc<dyn GeometricShape>> = Vec::new();
        let area_symbol = match node.search_child_by_name("area_symbol"){
            Some(an_area_symbol) => an_area_symbol,
            None => {eprintln!("I was not able to fine the 'area_symbol' subnode in an AreaSymbol node? WTF?"); return None}
        };
        let inner_color = match area_symbol.search_attribute_by_name("inner_color"){
            Some(s) => {
                match s.parse(){
                    Ok(an_u32) => Some(an_u32),
                    Err(_) => None, // It means that inner_color is "-1"
                }},
            None =>  {eprintln!("I was not able to fine the 'inner_color' attribute in an area_symbol node? WTF?"); return None}
        };
        if inner_color.is_some(){
            let area_symbol = Area{
                color: inner_color.unwrap(),
                nodes: Vec::new(),
            };
            geometric_shapes.push(Rc::new(area_symbol));
        }
        
        Some(Box::new(AreaSymbol{
            id: basic_symbol.id.clone(),
            code: basic_symbol.code.clone(),
            name: basic_symbol.name.clone(),
            description: basic_symbol.description.clone(),
            geometric_shapes: geometric_shapes,
            inner_color: inner_color,
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