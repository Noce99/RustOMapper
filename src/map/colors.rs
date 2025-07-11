use crate::map_file::reading::Node;
use std::rc::Rc;


pub struct Color {
    priority: u32,
    name: String,
    r:  u8,
    g:  u8,
    b:  u8,
}

pub struct ColorsBag {
    // There we are creating a Vector of Box.
    pub bag: Vec<Color>
}

impl ColorsBag {
    pub fn new(a_colors_node: Rc<Node>) -> Option<Self> {
        if a_colors_node.name != "colors" {
            return None
        }
        let mut a_bag_of_color: Vec<Color> = Vec::new();
        for child in a_colors_node.children.borrow().iter() {
            let priority: u32;
            match child.search_attribute_by_name("priority"){
                None => {println!("Not able to find 'priority' attribute in this Node!"); println!("{}", child); return None}
                Some(a_priority) => {priority = a_priority.parse().unwrap()}
            }
            let name: String;
            match child.search_attribute_by_name("name"){
                None => {println!("Not able to find 'name' attribute in this Node!"); println!("{}", child); return None}
                Some(a_name) => {name = a_name}
            }
            let rgb_option: Option<Rc<Node>> = child.search_child_by_name("rgb");
            let rgb: Rc<Node>;
            match rgb_option {
                None => {println!("Not possible to fina 'rgb' child in the current Node!"); println!("{}", child); return None}
                Some(a_rgb) => {rgb = a_rgb}
            }

            let r: f32;
            match rgb.search_attribute_by_name("r"){
                None => {println!("Not able to find 'r' in this Node!"); println!("{}", rgb); return None}
                Some(a_r) => {r = a_r.parse().unwrap()}
            }
            let g: f32;
            match rgb.search_attribute_by_name("g"){
                None => {println!("Not able to find 'g' in this Node!"); println!("{}", rgb); return None}
                Some(a_g) => {g = a_g.parse().unwrap()}
            }
            let b: f32;
            match rgb.search_attribute_by_name("b"){
                None => {println!("Not able to find 'b' in this Node!"); println!("{}", rgb); return None}
                Some(a_b) => {b = a_b.parse().unwrap()}
            }
            let r: u8 = (r*255.) as u8;
            let g: u8 = (g*255.) as u8;
            let b: u8 = (b*255.) as u8;
            a_bag_of_color.push(
                Color {
                    priority,
                    name,
                    r,
                    g,
                    b,
                }
            )
        }
        Some(Self { bag: a_bag_of_color })
    }
    pub fn insert(&mut self, color: Color) {
        self.bag.push(color);
    }
    pub fn len(&self) -> usize {
        self.bag.len()
    }
    pub fn show(&self) {
        for color in self.bag.iter() {
            println!("{} ({}) [{}; {}; {}]", color.name, color.priority, color.r, color.g, color.b);
        }
    }
}