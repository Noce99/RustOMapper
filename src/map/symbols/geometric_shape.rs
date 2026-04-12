use std::any::Any;
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use crate::map::elements::PointNode;
use crate::map::elements::ElementsBag;
use crate::map::symbols::SymbolsBag;

pub trait GeometricShape: Any {
    fn get_color(&self) -> u32;
    // fn get_type(&self) -> &str;
    fn move_by(& self, x: i64, y: i64) -> Rc<dyn GeometricShape>;
    // fn clone_rc(&self) -> Rc<dyn GeometricShape>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ring {
    pub x: i64,
    pub y: i64,
    pub inner_radius: u32,
    pub outer_width: u32,
    pub color: u32,
}

impl GeometricShape for Ring {
    fn get_color(&self) -> u32{
        self.color
    }
    // fn get_type(&self) -> &str{
    //     "ring"
    // }
    fn move_by(& self, x: i64, y: i64) -> Rc<dyn GeometricShape>{
        let mut to_return = self.clone();
        to_return.x += x;
        to_return.y += y;
        Rc::new(to_return)
    }
    // fn clone_rc(&self) -> Rc<dyn GeometricShape>{
    //     Rc::new(self.clone())
    // }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Circle {
    pub x: i64,
    pub y: i64,
    pub radius: u32,
    pub color: u32,
}

impl GeometricShape for Circle {
    fn get_color(&self) -> u32{
        self.color
    }
    // fn get_type(&self) -> &str{
    //     "circle"
    // }
    fn move_by(& self, x: i64, y: i64) -> Rc<dyn GeometricShape>{
        let mut to_return = self.clone();
        to_return.x += x;
        to_return.y += y;
        Rc::new(to_return)
    }
    // fn clone_rc(&self) -> Rc<dyn GeometricShape>{
    //     Rc::new(self.clone())
    // }
}



#[derive(Serialize, Deserialize, Clone)]
pub struct Line {
    pub color: u32,
    pub line_width: u32,
    pub nodes: Vec<Node>,
}

impl GeometricShape for Line{
    fn get_color(&self) -> u32{
        self.color
    }
    // fn get_type(&self) -> &str{
    //     "line"
    // }
    fn move_by(& self, x: i64, y: i64) -> Rc<dyn GeometricShape>{
        let mut to_return = self.clone();
        for i in 0..to_return.nodes.len() {
            to_return.nodes[i].point.x += x;
            to_return.nodes[i].point.y += y;
            match & to_return.nodes[i].left_branch{
                Some(left_branch) => {
                    to_return.nodes[i].left_branch = Some(
                        Point{
                            x: left_branch.x + x,
                            y: left_branch.y + y,
                        }
                    )
                }
                None => {}
            }
            match & to_return.nodes[i].right_branch{
                Some(right_branch) => {
                    to_return.nodes[i].right_branch = Some(
                        Point{
                            x: right_branch.x + x,
                            y: right_branch.y + y,
                        }
                    )
                }
                None => {}
            }
        }
        Rc::new(to_return)
    }
    // fn clone_rc(&self) -> Rc<dyn GeometricShape>{
    //     Rc::new(self.clone())
    // }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Area{
    pub color: u32,
}

impl GeometricShape for Area{
    fn get_color(&self) -> u32{
        self.color
    }
    // fn get_type(&self) -> &str{
    //     "area"
    // }
    fn move_by(& self, _x: i64, _y: i64) -> Rc<dyn GeometricShape>{
        Rc::new(self.clone())
    }
    // fn clone_rc(&self) -> Rc<dyn GeometricShape>{
    //     Rc::new(self.clone())
    // }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    pub point: Point,
    pub left_branch: Option<Point>,
    pub right_branch: Option<Point>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

pub fn from_number_to_vec_of_nodes(coordinates : Vec<PointNode>) -> Vec<Node>{
    let mut nodes = Vec::new();
    let mut i : usize = 0;
    let mut a_left_branch : Option<Point> = None;
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
            // Bayesian
            nodes.push(
                crate::map::symbols::geometric_shape::Node {
                    point: Point {
                        x: coordinates[i].x,
                        y: coordinates[i].y,
                    },
                    left_branch: a_left_branch,
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
    nodes
}

pub struct GeometricShapesBag{
    pub bag: Vec<Rc<dyn GeometricShape>>
}

impl GeometricShapesBag{
    pub fn from_elements_bag(elements_bag : & ElementsBag, symbols : & SymbolsBag) -> GeometricShapesBag{
        let mut bag : Vec<Rc<dyn GeometricShape>> = Vec::new();        
        let mut max_color_id : u32 = 0;
        for element in & elements_bag.bag {
            let symbol = symbols.symbol_by_id(element.symbol);
            
            match symbol{
                Some(a_symbol) => {
                    // println!("Element Symbol = {0}, Element Type = {1}", symbol.get_name(), symbol.get_symbol_type());
                    for geometric_shape in a_symbol.get_geometric_shapes(){
                        // println!("\tType of Geometric Shape = {0}", geometric_shape.get_type());
                        if a_symbol.get_symbol_type() == "punctual"{
                            let new_geometric_shape = geometric_shape.move_by(element.coordinates[0].x, element.coordinates[0].y);
                            if new_geometric_shape.get_color() > max_color_id{
                                max_color_id = new_geometric_shape.get_color();
                            }
                            bag.push(new_geometric_shape)
                        }/*elif a_symbol.get_symbol_type() == "linear"{

                        }elif a_symbol.get_symbol_type() == "area"{
                        
                        }*/
                    }
                }
                None => {}
            }
        }

        bag.sort_by_key(|item| max_color_id - item.get_color());

        // println!("Just geometric shapes:");
        // for geometric_shape in & bag{
        //     println!("\tType of Geometric Shape = {0}", geometric_shape.get_color());
        // }
        GeometricShapesBag{ bag: bag }
    }
}

