use std::any::Any;
use std::ptr::addr_eq;
use std::rc::Rc;
use crate::map_file::reading::Node;

pub trait GeometricShape: Any{
    fn render(&self);
}

// CIRCLE
pub struct Annulus {
    pub inner_radius: u32,
    pub inner_color: Option<u32>,
    pub outer_width: u32,
    pub outer_color: Option<u32>,
    pub elements: u32,
}

impl Annulus {
    pub fn new_from_node(node: Rc<Node>) -> Option<Self>{
        let inner_radius_option = node.search_attribute_by_name("inner_radius");
        let inner_radius: u32;
        match inner_radius_option{
            Some(a_inner_radius) => {inner_radius = a_inner_radius.parse().unwrap()}
            None => {println!("I was not able to fine an 'inner_radius' attribute in a point symbol node? WTF?"); return None}
        }
        let inner_color_option = node.search_attribute_by_name("inner_color");
        let inner_color: Option<u32>;
        match inner_color_option{
            Some(a_inner_color) => {
                if a_inner_color == "-1"{
                    inner_color = None;
                }else{
                    inner_color = Some(a_inner_color.parse().unwrap())
                }
            }
            None => {println!("I was not able to fine an 'inner_color' attribute in a point symbol node? WTF?"); return None}
        }
        let outer_width_option = node.search_attribute_by_name("outer_width");
        let outer_width: u32;
        match outer_width_option{
            Some(a_outer_width) => {outer_width = a_outer_width.parse().unwrap()}
            None => {println!("I was not able to fine an 'outer_width' attribute in a point symbol node? WTF?"); return None}
        }
        let outer_color_option = node.search_attribute_by_name("outer_color");
        let outer_color: Option<u32>;
        match outer_color_option{
            Some(an_outer_color) => {
                if an_outer_color == "-1" {
                    outer_color = None;
                }else{
                    outer_color = Some(an_outer_color.parse().unwrap());
                }
            }
            None => {println!("I was not able to fine an 'outer_color' attribute in a point symbol node? WTF?"); return None}
        }
        let elements_option = node.search_attribute_by_name("elements");
        let elements: u32;
        match elements_option{
            Some(a_elements) => {elements = a_elements.parse().unwrap()}
            None => {println!("I was not able to fine an 'elements' attribute in a point symbol node? WTF?"); return None}
        }
        Some(Annulus {
            inner_radius,
            inner_color,
            outer_width,
            outer_color,
            elements
        })
    }
}

impl GeometricShape for Annulus {
    fn render(&self){

    }
}

// LINE
pub struct Line {

}

impl GeometricShape for Line{
    fn render(&self){

    }
}

// AREA
pub struct Area{

}

impl GeometricShape for Area{
    fn render(&self){

    }
}