use std::rc::Rc;
use serde::{Deserialize, Serialize};
use crate::map_file::reading::Node;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PointNode {
    pub x: i64,
    pub y: i64,
    pub bayesian: bool,
    pub back_to_start: bool,
}
impl PointNode {
    pub fn new_from_string(s: &str) -> PointNode {
        let numbers: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let x = numbers[0];
        let y = numbers[1];
        let mut bayesian = false;
        let mut back_to_start = false;
        if numbers.len() == 3 {
            if numbers[2] == 1 {
                bayesian = true;
            }else{
                back_to_start = true;
            }
        }
        PointNode { x, y, bayesian, back_to_start}
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
    pub min_x: Option<i64>,
    pub max_x: Option<i64>,
    pub min_y: Option<i64>,
    pub max_y: Option<i64>,
}

impl Element {
    pub fn elements_from_object_node(node: Rc<Node>) -> Element {
        let element_type = node.search_attribute_by_name("type").unwrap().parse::<u32>().unwrap();
        let symbol = node.search_attribute_by_name("symbol").unwrap().parse::<i32>().unwrap();
        let coordinate_str_node = node.search_child_by_name("coords").unwrap();
        let inner_text_str = coordinate_str_node.inner_text.borrow().clone().unwrap();
        let coordinates_vec_str: Vec<&str> = inner_text_str.split(';').collect();
        let mut coordinates: Vec<PointNode> = Vec::new();
        let mut min_x = None;
        let mut max_x = None;
        let mut min_y = None;
        let mut max_y = None;
        if coordinates_vec_str.len() >= 1{
            for item in coordinates_vec_str {
                if item != "" {
                    let a_point: PointNode = PointNode::new_from_string(&item);
                    if min_x.is_none(){
                        min_x = Some(a_point.x);
                        max_x = Some(a_point.x);
                        min_y = Some(a_point.y);
                        max_y = Some(a_point.y);
                    }else{
                        if a_point.x < min_x.unwrap(){
                            min_x = Some(a_point.x);
                        } else if a_point.x > max_x.unwrap(){
                            max_x = Some(a_point.x);
                        }
                        if a_point.y < min_y.unwrap(){
                            min_y = Some(a_point.y);
                        }else if a_point.y > max_y.unwrap(){
                            max_y = Some(a_point.y);
                        }
                    }
                    coordinates.push(a_point);
                }
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
            min_x,
            max_x,
            min_y,
            max_y
        }
    }

    pub fn change_origin(&mut self, x : i64, y : i64){
        for i in 0..self.coordinates.len(){
            self.coordinates[i].x -= x;
            self.coordinates[i].y -= y;
        }
        if self.min_x.is_some(){
            self.min_x = Some(self.min_x.unwrap() - x);
            self.max_x = Some(self.max_x.unwrap() - x);
            self.min_y = Some(self.min_y.unwrap() - y);
            self.max_y = Some(self.max_y.unwrap() - y);
        }
        self.pattern_rotation_x -= x;
        self.pattern_rotation_y -= y;
    }
}

pub struct ElementsBag{
    pub bag: Vec<Element>,
    pub min_x: Option<i64>,
    pub max_x: Option<i64>,
    pub min_y: Option<i64>,
    pub max_y: Option<i64>,
}
impl ElementsBag {
    pub fn elements_from_parts(node:  Rc<Node>) -> ElementsBag {
        let mut elements: Vec<Element> = Vec::new();
        let mut min_x = None;
        let mut max_x = None;
        let mut min_y = None;
        let mut max_y = None;
        for part in node.children.borrow().iter(){
            if part.name != "part"{
                eprintln!("A child of a node without name 'part'! {}", part.name);
            }
            let objects = part.search_child_by_name("objects").unwrap();
            for object in objects.children.borrow().iter() {
                let an_element = Element::elements_from_object_node(Rc::clone(&object));
                if min_x.is_none(){
                    min_x = Some(an_element.min_x.unwrap());
                    max_x = Some(an_element.max_x.unwrap());
                    min_y = Some(an_element.min_y.unwrap());
                    max_y = Some(an_element.max_y.unwrap());
                }else{
                    if an_element.min_x.unwrap() < min_x.unwrap(){
                        min_x = Some(an_element.min_x.unwrap());
                    }
                    if an_element.max_x.unwrap() > max_x.unwrap(){
                        max_x = Some(an_element.max_x.unwrap());
                    }
                    if an_element.min_y.unwrap() < min_y.unwrap(){
                        min_y = Some(an_element.min_y.unwrap());
                    }
                    if an_element.max_y.unwrap() > max_y.unwrap(){
                        max_y = Some(an_element.max_y.unwrap());
                    }
                }
                elements.push(an_element);
            }
        }
        ElementsBag {
            bag: elements,
            min_x,
            max_x,
            min_y,
            max_y
        }
    }
    pub fn change_origin(&mut self, x : i64, y : i64){
        for i in 0..self.bag.len(){
            self.bag[i].change_origin(x, y);
        }
        if self.min_x.is_some(){
            self.min_x = Some(self.min_x.unwrap() - x);
            self.max_x = Some(self.max_x.unwrap() - x);
            self.min_y = Some(self.min_y.unwrap() - y);
            self.max_y = Some(self.max_y.unwrap() - y);
        }
    }

    pub fn normalize_origin(&mut self){
        self.change_origin(self.min_x.unwrap(), self.min_y.unwrap());
    }
    
}