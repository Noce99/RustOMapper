use std::any::Any;
use std::ptr::addr_eq;
use std::rc::Rc;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use crate::map::elements::PointNode;
use crate::map::elements::ElementsBag;
use crate::map::symbols::SymbolsBag;

pub trait GeometricShape: Any{
    fn get_color(&self) -> u32;
    fn get_type(&self) -> &str;
}

#[derive(Serialize, Deserialize)]
pub struct Ring {
    pub x: i32,
    pub y: i32,
    pub inner_radius: u32,
    pub outer_width: u32,
    pub color: u32,
}

impl GeometricShape for Ring {
    fn get_color(&self) -> u32{
        self.color
    }
    fn get_type(&self) -> &str{
        "ring"
    }
}

#[derive(Serialize, Deserialize)]
pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: u32,
    pub color: u32,
}

impl GeometricShape for Circle {
    fn get_color(&self) -> u32{
        self.color
    }
    fn get_type(&self) -> &str{
        "circle"
    }
}



#[derive(Serialize, Deserialize)]
pub struct Line {
    pub color: u32,
    pub line_width: u32,
    pub nodes: Vec<Node>,
}

impl GeometricShape for Line{
    fn get_color(&self) -> u32{
        self.color
    }
    fn get_type(&self) -> &str{
        "line"
    }
}

#[derive(Serialize, Deserialize)]
pub struct Area{
    pub color: u32,
}

impl GeometricShape for Area{
    fn get_color(&self) -> u32{
        self.color
    }
    fn get_type(&self) -> &str{
        "area"
    }
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

        let start_shapes_bag_creation_time = Instant::now();

        for element in & elements_bag.bag {
            let symbol = symbols.symbol_by_id(element.symbol).unwrap();
            
            println!("Element Symbol = {0}, Element Type = {1}", symbol.get_name(), symbol.get_symbol_type());
            for geometric_shape in symbol.get_geometric_shapes(){
                println!("\tType of Geometric Shape = {0}", geometric_shape.get_type());
                bag.push(Rc::clone(&geometric_shape))
            }
        }
        println!("Just geometric shapes:");
        for geometric_shape in & bag{
            println!("\tType of Geometric Shape = {0}", geometric_shape.get_type());
        }
        // TODO: Order the geometric shape by color id to be ready to be rendered!
        let shapes_bag_time = start_shapes_bag_creation_time.elapsed();
        let shapes_bag_time_s = shapes_bag_time.as_secs();
        let shapes_bag_time_ms = shapes_bag_time.subsec_millis();
        println!("Created the Geometric Shapes Bag in {shapes_bag_time_s} s and \
         {shapes_bag_time_ms} ms.");
        GeometricShapesBag{ bag: bag }
    }
}

