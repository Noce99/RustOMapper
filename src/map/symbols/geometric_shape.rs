use std::any::Any;
use std::ptr::addr_eq;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

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