use std::any::Any;
use std::ptr::addr_eq;
use std::rc::Rc;
use crate::map_file::reading::Node;

pub trait GeometricShape: Any{}

// CIRCLE
pub struct Ring {
    pub inner_radius: u32,
    pub outer_width: u32,
    pub color: u32,
}

impl GeometricShape for Ring {}

pub struct Circle {
    pub radius: u32,
    pub color: u32,
}

impl GeometricShape for Circle {}



// LINE
pub struct Line {

}

impl GeometricShape for Line{
}

// AREA
pub struct Area{

}

impl GeometricShape for Area{
}