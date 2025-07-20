use std::rc::Rc;
use crate::map::symbols::{area, linear, punctual, text, Symbol, SymbolCommon};
use crate::map::symbols::geometric_shape::{Ring, GeometricShape, Circle};
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
        let point_symbol_node_option = node.search_child_by_name("point_symbol");
        let point_symbol_node: Rc<Node>;
        match point_symbol_node_option {
            Some(a_node) => {point_symbol_node = a_node}
            None => {println!("I was not able to fine the 'point symbol' child in this symbol: {:?}", basic_symbol); return None}
        }

        let mut geometric_shapes : Vec<Box<dyn GeometricShape>> = Vec::new();
        let (ring_option, circle_option) = from_point_symbol_node_to_circle_ring(point_symbol_node.clone());
        if ring_option.is_some(){
            geometric_shapes.push(Box::new(ring_option.unwrap()));
        }
        if circle_option.is_some(){
            geometric_shapes.push(Box::new(circle_option.unwrap()));
        }

        // In the point definition itself there is a definition of a geometric shape part of the symbol


        for child in point_symbol_node.children.borrow().iter(){
            if child.name != "element"{
                eprintln!("In a symbol there was a child named {} that I was not expecting, continuing...", child.name);
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
                let inner_point_symbol_option = symbol_node.search_child_by_name("point_symbol");
                let inner_point_symbol_node: Rc<Node>;
                match inner_point_symbol_option{
                    Some(a_node) => {inner_point_symbol_node = a_node}
                    None => {eprintln!("Inside an element of type {} I cannot find a point_symbol?!", basic_geometric_symbol.symbol_type); return None}
                }
                let (ring_option, circle_option) = from_point_symbol_node_to_circle_ring(inner_point_symbol_node.clone());
                if ring_option.is_some(){
                    geometric_shapes.push(Box::new(ring_option.unwrap()));
                }
                if circle_option.is_some(){
                    geometric_shapes.push(Box::new(circle_option.unwrap()));
                }
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

fn from_point_symbol_node_to_circle_ring(node: Rc<Node>) -> (Option<Ring>, Option<Circle>){
    let inner_radius_option = node.search_attribute_by_name("inner_radius");
    let inner_radius: u32;
    match inner_radius_option {
        Some(a_inner_radius) => {inner_radius = a_inner_radius.parse().unwrap();}
        None => {println!("I was not able to fine the attribute 'inner_radius' in a point_symbol?"); return (None, None)}
    }
    let inner_color_option = node.search_attribute_by_name("inner_color");
    let inner_color: Option<u32>;
    match inner_color_option {
        Some(a_inner_color) => {
            if a_inner_color == "-1"{
                inner_color = None;
            }else {
                inner_color = Some(a_inner_color.parse().unwrap());
            }
        }
        None => {println!("I was not able to fine the attribute 'inner_color' in a point_symbol?"); return (None, None)}
    }
    let outer_width_option = node.search_attribute_by_name("outer_width");
    let outer_width: u32;
    match outer_width_option {
        Some(a_outer_width) => {outer_width = a_outer_width.parse().unwrap()}
        None => {println!("I was not able to fine the attribute 'outer_width' in a point_symbol?"); return (None, None)}
    }
    let outer_color_option = node.search_attribute_by_name("outer_color");
    let outer_color: Option<u32>;
    match outer_color_option {
        Some(an_outer_color) => {
            if an_outer_color == "-1"{
                outer_color = None;
            }else {
                outer_color = Some(an_outer_color.parse().unwrap());
            }
        }
        None => {println!("I was not able to fine the attribute 'outer_color' in a point_symbol?"); return (None, None)}
    }
    let ring: Option<Ring>;
    let circle: Option<Circle>;
    if inner_radius == 0 {
        ring = None;
        if outer_width == 0 || outer_color.is_none(){
            // inner_radius = 0
            // outer_width == 0 || outer_color.is_none()
            circle = None;
        }else{
            // inner_radius = 0
            // outer_width > 0 && outer_color.is_some()
            circle = Some(Circle{
                radius: outer_width,
                color: outer_color.unwrap(),
            });
        }
    } else {
        if inner_color.is_none(){
            // inner_radius > 0
            // inner_color.is_none()
            circle = None;
        }else {
            // inner_radius > 0
            // inner_color.is_some()
            circle = Some(Circle {
                radius: inner_radius,
                color: inner_color.unwrap(),
            });
        }
        if outer_width == 0 || outer_color.is_none(){
            // inner_radius > 0
            // outer_width == 0 || outer_color.is_none()
            ring = None;
        }else{
            // inner_radius > 0
            // outer_width > 0 && outer_color.is_some()
            ring = Some(Ring{
                inner_radius,
                outer_width,
                color: outer_color.unwrap(),
            })
        }
    }
    (ring, circle)
}