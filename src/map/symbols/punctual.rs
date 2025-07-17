use std::rc::Rc;
use crate::map::symbols::{area, linear, punctual, text, Symbol, SymbolCommon};
use crate::map::symbols::geometric_shape::{Annulus, GeometricShape};
use crate::map_file::reading::Node;

pub struct PunctualSymbol{
    pub id: u32,
    pub code: String,
    pub name: String,
    pub description: String,
    pub geometric_shapes: Vec<Box<dyn GeometricShape>>,
}

impl PunctualSymbol{
    pub fn symbol_from_a_node(basic_symbol: &SymbolCommon, node: &Rc<Node>) -> Option<Box<Self>> {
        let point_symbol_option = node.search_child_by_name("point_symbol");
        let point_symbol_node: Rc<Node>;
        match point_symbol_option{
            Some(a_node) => {point_symbol_node = a_node}
            None => {println!("I was not able to fine the 'point symbol' child in this symbol: {:?}", basic_symbol); return None}
        }
        let mut geometric_shapes : Vec<Box<dyn GeometricShape>> = Vec::new();
        for child in point_symbol_node.children.borrow().iter(){
            if child.name != "element"{
                eprintln!("In a symbol there was a child named {}", child.name);
                continue;
            }
            let symbol_option = child.search_child_by_name("symbol");
            let symbol_node: Rc<Node>;
            match symbol_option{
                Some(a_node) => {symbol_node = a_node}
                None => {println!("I was not able to fine the 'symbol' child in this element: {}", child); return None}
            }

            let basic_geometric_symbol_option = SymbolCommon::new_from_node(&symbol_node);
            let basic_geometric_symbol;
            match basic_geometric_symbol_option{
                Some(a) => {basic_geometric_symbol = a}
                None => {eprintln!("I was not able to build a SymbolCommon from \n{}!", symbol_node); return None}
            }
            if basic_geometric_symbol.symbol_type == "2"{

                //geometric_shapes.push(linear_symbol);
            }else if basic_geometric_symbol.symbol_type == "4" || basic_symbol.symbol_type == "16" {

                //geometric_shapes.push(area_symbol);
            }else if basic_geometric_symbol.symbol_type == "1" {
                let point_symbol_option = node.search_child_by_name("point_symbol");
                let point_symbol_node: Rc<Node>;
                match point_symbol_option{
                    Some(a_node) => {point_symbol_node = a_node}
                    None => {eprintln!("Inside an element of type {} I cannot find a point_symbol?!", basic_geometric_symbol.symbol_type); return None}
                }
                let circle_option = Annulus::new_from_node(point_symbol_node);
                let circle;
                match circle_option{
                    Some(a_circle) => {circle = a_circle}
                    None => {eprintln!("I was not able to create a LinearSymbol from a Symbol Node. [name = {}, type = {}]", basic_symbol.name.clone(), basic_symbol.symbol_type.clone()); return None}
                }
                geometric_shapes.push(Box::new(circle));
            }else if basic_geometric_symbol.symbol_type == "8" {

                //geometric_shapes.push(text_symbol);
            }else{
                println!("hi, it's me hehehe ... Find out a strange type for symbol ({}) [type={}]", basic_symbol.name, basic_symbol.symbol_type);
            }


        }
        Some(Box::new(PunctualSymbol{
            id: basic_symbol.id.clone(),
            code: basic_symbol.code.clone(),
            name: basic_symbol.name.clone(),
            description: basic_symbol.description.clone(),
            geometric_shapes,
        }))
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