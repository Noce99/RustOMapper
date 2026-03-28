use std::any::Any;
use std::ptr::addr_eq;
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use crate::map::elements::PointNode;


pub trait GeometricShape: Any{}

#[derive(Serialize, Deserialize)]
pub struct Ring {
    pub x: i32,
    pub y: i32,
    pub inner_radius: u32,
    pub outer_width: u32,
    pub color: u32,
}

impl GeometricShape for Ring {}

#[derive(Serialize, Deserialize)]
pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: u32,
    pub color: u32,
}

impl GeometricShape for Circle {}



#[derive(Serialize, Deserialize)]
pub struct Line {
    pub color: u32,
    pub line_width: u32,
    pub nodes: Vec<Node>,
}

impl GeometricShape for Line{
}

#[derive(Serialize, Deserialize)]
pub struct Area{

}

impl GeometricShape for Area{
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

