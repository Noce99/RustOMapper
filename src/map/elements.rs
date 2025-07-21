use std::num::ParseIntError;
use std::process::exit;
use std::rc::Rc;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use crate::map_file::reading::Node;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PointNode {
    pub x: i64,
    pub y: i64,
    pub three_item: bool,
}
impl PointNode {
    pub fn new_from_string(s: &str) -> PointNode {
        let numbers: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let x = numbers[0];
        let y = numbers[1];
        let mut three_item = false;
        if numbers.len() == 3 {
            three_item = true;
        }
        PointNode { x, y, three_item }
    }
}

#[derive(Debug, Clone)]
pub struct Element {
    pub element_type: u32,
    pub symbol: i32,
    pub coordinates: Vec<PointNode>,
    pub pattern_rotation: f32,
    pub pattern_rotation_x: i64,
    pub pattern_rotation_y: i64,
}

impl Element {
    pub fn elements_from_object_node(node: Rc<Node>) -> Element {
        let element_type = node.search_attribute_by_name("type").unwrap().parse::<u32>().unwrap();
        let symbol = node.search_attribute_by_name("symbol").unwrap().parse::<i32>().unwrap();
        let coordinate_str_node = node.search_child_by_name("coords").unwrap();
        let inner_text_str = coordinate_str_node.inner_text.borrow().clone().unwrap();
        let coordinates_vec_str: Vec<&str> = inner_text_str.split(';').collect();
        let mut coordinates: Vec<PointNode> = Vec::new();
        for item in coordinates_vec_str {
            if item != "" {
                coordinates.push(PointNode::new_from_string(&item));
            }
        }
        let mut pattern_rotation: f32 = 0.;
        let mut pattern_rotation_x: i64 = 0;
        let mut pattern_rotation_y: i64 = 0;
        if let Some(pattern_node) = node.search_child_by_name("pattern"){
            pattern_rotation = pattern_node.search_attribute_by_name("rotation").unwrap()
                .parse::<f32>().unwrap();
            let pattern_coord = pattern_node.search_child_by_name("coord").unwrap();
            pattern_rotation_x = pattern_coord.search_attribute_by_name("x").unwrap()
                .parse::<i64>().unwrap();
            pattern_rotation_y = pattern_coord.search_attribute_by_name("y").unwrap()
                .parse::<i64>().unwrap();
        }

        Element {
            element_type,
            symbol,
            coordinates,
            pattern_rotation,
            pattern_rotation_x,
            pattern_rotation_y,
        }
    }
}

pub struct ElementsBag{
    pub bag: Vec<Element>,
}
impl ElementsBag {
    pub fn elements_from_parts(node:  Rc<Node>) -> ElementsBag {
        println!("I'm going to create an Elements Bag!");
        let start_elements_bag_creation_time = Instant::now();
        let mut elements: Vec<Element> = Vec::new();
        for part in node.children.borrow().iter(){
            if part.name != "part"{
                eprintln!("A child of a node without name 'part'! {}", part.name);
            }
            let objects = part.search_child_by_name("objects").unwrap();
            for object in objects.children.borrow().iter() {
                elements.push(Element::elements_from_object_node(Rc::clone(&object)));
            }
        }
        let elements_bag_time = start_elements_bag_creation_time.elapsed();
        let elements_bag_time_s = elements_bag_time.as_secs();
        let elements_bag_time_ms = elements_bag_time.subsec_millis();
        println!("Created the Elements Bag in {elements_bag_time_s} s and \
         {elements_bag_time_ms} ms.");
        ElementsBag {bag: elements}
    }
}