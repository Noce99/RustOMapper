use std::rc::Rc;
use crate::map::elements::PointNode;
use crate::map::symbols::{area, linear, punctual, text, Symbol, SymbolCommon};
use crate::map::symbols::geometric_shape::{Ring, GeometricShape, Circle, Point, Line};
use crate::map_file::reading::Node;

pub struct PunctualSymbol{
    pub id: i32,
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
        let (ring_option, circle_option) = from_point_symbol_node_to_circle_ring(point_symbol_node.clone(), 0, 0);
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
            let object_node = child.search_child_by_name("object").unwrap();
            let coordinate_str_node = object_node.search_child_by_name("coords").unwrap();
            let inner_text_str = coordinate_str_node.inner_text.borrow().clone().unwrap();
            let coordinates_vec_str: Vec<&str> = inner_text_str.split(';').collect();
            let mut coordinates: Vec<PointNode> = Vec::new();
            for item in coordinates_vec_str {
                if item != "" {
                    coordinates.push(PointNode::new_from_string(&item));
                }
            }

            let basic_geometric_symbol_option = SymbolCommon::new_from_node(&symbol_node);
            let basic_geometric_symbol;
            match basic_geometric_symbol_option{
                Some(a) => {basic_geometric_symbol = a}
                None => {eprintln!("I was not able to build a SymbolCommon from \n{}!", symbol_node); return None}
            }
            if basic_geometric_symbol.symbol_type == "2"{
                let inner_linear_symbol_option = symbol_node.search_child_by_name("line_symbol");
                let inner_linear_symbol_node: Rc<Node>;
                match inner_linear_symbol_option{
                    Some(a_node) => {inner_linear_symbol_node = a_node}
                    None => {eprintln!("Inside an element of type {} I cannot find a line_symbol?!", basic_geometric_symbol.symbol_type); return None}
                }
                let color_option = inner_linear_symbol_node.search_attribute_by_name("color");
                let color: u32;
                match color_option {
                    Some(a_color) => {
                        if a_color.parse::<i32>().unwrap() < 0 {
                            continue;
                        }
                        color = a_color.parse().unwrap();
                    }
                    None => {println!("I was not able to fine the attribute 'color' in a line_symbol?"); return None}
                }
                let line_width_option = inner_linear_symbol_node.search_attribute_by_name("line_width");
                let line_width: u32;
                match line_width_option {
                    Some(a_line_width) => {line_width = a_line_width.parse().unwrap();}
                    None => {println!("I was not able to fine the attribute 'line_width' in a line_symbol?"); return None}
                }
                let mut nodes:Vec<crate::map::symbols::geometric_shape::Node> = Vec::new();
                let mut i:usize = 0;
                let mut a_left_branch:Option<Point> = None;
                loop{
                    if ! coordinates[i].bayesian {
                        nodes.push(
                            crate::map::symbols::geometric_shape::Node {
                                point: Point {
                                    x: coordinates[i].x,
                                    y: coordinates[i].y,
                                },
                                left_branch: a_left_branch,
                                right_branch: None,
                            }
                        );
                        a_left_branch = None;
                        i += 1;
                    }else{
                        nodes.push(
                            crate::map::symbols::geometric_shape::Node {
                                point: Point {
                                    x: coordinates[i].x,
                                    y: coordinates[i].y,
                                },
                                left_branch: None,
                                right_branch: Some(Point{
                                    x: coordinates[i+1].x,
                                    y: coordinates[i+1].y,
                                }),
                            }
                        );
                        a_left_branch = Some(Point{
                            x: coordinates[i+2].x,
                            y: coordinates[i+2].y,
                        });
                        i = i+3;
                    }
                    if i >= coordinates.len(){
                        break;
                    }
                }
                let linear_symbol = Line{
                    color,
                    line_width,
                    nodes,
                };
                geometric_shapes.push(Box::new(linear_symbol));
            }else if basic_geometric_symbol.symbol_type == "4" || basic_symbol.symbol_type == "16" {

                //geometric_shapes.push(Box::new(area_symbol));
            }else if basic_geometric_symbol.symbol_type == "1" {
                assert_eq!(coordinates.len(), 1);
                assert!(!coordinates[0].bayesian);
                let x = coordinates[0].x;
                let y = coordinates[0].y;
                let inner_point_symbol_option = symbol_node.search_child_by_name("point_symbol");
                let inner_point_symbol_node: Rc<Node>;
                match inner_point_symbol_option{
                    Some(a_node) => {inner_point_symbol_node = a_node}
                    None => {eprintln!("Inside an element of type {} I cannot find a point_symbol?!", basic_geometric_symbol.symbol_type); return None}
                }
                let (ring_option, circle_option) = from_point_symbol_node_to_circle_ring(inner_point_symbol_node.clone(), x, y);
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

    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_symbol_type(&self) -> String {
        "punctual".to_string()
    }
}

fn from_point_symbol_node_to_circle_ring(node: Rc<Node>, x :i64, y :i64) -> (Option<Ring>, Option<Circle>){
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
                x: x as i32,
                y: y as i32,
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
                x: x as i32,
                y: y as i32,
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
                x: x as i32,
                y: y as i32,
                inner_radius,
                outer_width,
                color: outer_color.unwrap(),
            })
        }
    }
    (ring, circle)
}