//! This file contains:
//! The definition and implementation of The Color and ColorsBag structs

use crate::map_file::reading::Node;
use std::rc::Rc;

/// This struct describe a Color
pub struct Color {
    // The priority is used to identify which color should be on top od which others
    pub priority: u32,
    // The name of the Color
    pub name: String,
    // The r, g and b values in the range [0; 255]
    pub r:  u8,
    pub g:  u8,
    pub b:  u8,
    // The opacity/alpha of the color, in the range [0.; 1.]
    pub opacity: f32,
}

impl Color{
    pub fn get_string_rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

/// Converts a CMYK color (each channel in [0; 1]) to RGB (each channel in [0; 255]).
/// Uses the same naive conversion as Qt's `QColor::fromCmykF` (r = (1-c)(1-k), ...).
fn cmyk_to_rgb(c: f32, m: f32, y: f32, k: f32) -> (u8, u8, u8) {
    let r = (1. - c) * (1. - k);
    let g = (1. - m) * (1. - k);
    let b = (1. - y) * (1. - k);
    ((r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8)
}

/// This struct is used ad the container (Bag) for all the colors defined in a .omap file
pub struct ColorsBag {
    // The colors that are used to create symbols
    pub colors: Vec<Color>,
    // The colors used to define other colors
    // pub spot_colors: Vec<Color>,
}

impl ColorsBag {
    pub fn new(a_colors_node: Rc<Node>) -> Option<Self> {
        /*
        Example:
        color (priority:"34", name:"Yellow 50%", c:"0", m:"0.135", y:"0.395", k:"0", opacity:"1", ) {}
            spotcolors (knockout:"true", ) {}
                component (factor:"0.5", spotcolor:"32", ) {}
            cmyk (method:"spotcolor", ) {}
            rgb (method:"spotcolor", r:"1", g:"0.865", b:"0.605", ) {}

        cmyk and rgb are always there. spotcolors can also not be there.
        The final color shown on the map is the cmyk definition in the attributes of `color`,
        unless cmyk's method is "rgb" (i.e. cmyk was derived from rgb), in which case the rgb
        values are used instead.
        Knockout true means that when this (spot) color overlaps another one, it completely
        erases/replaces the color underneath instead of blending/darkening on top of it
        (simulated overprinting).
        Both cmyk and rgb have a `method` attribute, one of:
          - "custom": the value is given directly by its own attributes (c/m/y/k or r/g/b)
          - "spotcolor": the value is computed from the referenced `spotcolors` composition
          - for cmyk only, "rgb": the value is derived by converting the rgb value
          - for rgb only, "cmyk": the value is derived by converting the cmyk value
        rgb always has r, g and b attributes, holding its value regardless of method.
        cmyk instead only has c, m, y and k attributes on the parent `color` node (not on the
        `cmyk` node itself), and those are only meaningful when cmyk's method is "custom".
        */
        if a_colors_node.name != "colors" {
            eprintln!("Parsing colors from a XML-Element that is not called colors?!");
            return None
        }
        let mut a_bag_of_color: Vec<Color> = Vec::new();
        for child in a_colors_node.children.borrow().iter() {
            let Some(attributes) = parse_color_attributes(child.clone()) else { continue };

            let cmyk = match child.search_child_by_name("cmyk") {
                Some(a_cmyk_node) => match parse_cmyk(a_cmyk_node){
                    Some(a_cmyk) => a_cmyk,
                    None => {eprintln!("Not possible to to parse an cmyk_node! {child}"); continue}
                },
                None => {eprintln!("Not possible to find a 'cmyk' child in the current Node! {child}"); continue}
            };
            let rgb = match child.search_child_by_name("rgb") {
                Some(a_rgb_node) => match parse_rgb(a_rgb_node){
                    Some(a_rgb) => a_rgb,
                    None => {eprintln!("Not possible to to parse an rgb_node! {child}"); continue}
                },
                None => {eprintln!("Not possible to find a 'rgb' child in the current Node! {child}"); continue}
            };

            
            a_bag_of_color.push(
                Color {
                    priority: attributes.priority,
                    name: attributes.name,
                    r,
                    g,
                    b,
                    opacity: attributes.opacity,
                }
            )
        }
        Some(Self { colors: a_bag_of_color })
    }

    pub fn len(&self) -> usize {
        self.colors.len()
    }
}


// Some helper functions
fn parse_float_attribute(node: &Node, attribute: &str) -> Option<f32> {
    let raw = node.search_attribute_by_name(attribute)?;
    raw.parse().ok()
}
fn parse_u8_attribute(node: &Node, attribute: &str) -> Option<u8> {
    let raw = node.search_attribute_by_name(attribute)?;
    let value: f32 = raw.parse().ok()?;
    Some((value * 255.) as u8)
}
fn parse_method(node: &Node) -> Option<Method>{
    let method_str = match node.search_attribute_by_name("method") {
        Some(a_method) => a_method,
        None => {eprintln!("Not able to find 'method' attribute in this Node! {node}"); return None}
    };
    let method = match method_str.as_str() {
        "custom" => Method::Custom,
        "spotcolor" => Method::SpotColor,
        "rgb" => Method::Rgb,
        _ => {eprintln!("A method is any of {{custom, spotcolor or rgb}} ! {method_str}"); return None}
    };
    return Some(method)
}
fn parse_color_attributes(color: Rc<Node>) -> Option<ColorAttributes>{
    let priority = match color.search_attribute_by_name("priority"){
        Some(a_priority) => match a_priority.parse(){
            Ok(b_priority) => b_priority,
            Err(_) => {eprintln!("Not able to parse 'priority' attribute in this Node! {color}"); return None}
        },
        None => {eprintln!("Not able to find 'priority' attribute in this Node! {color}"); return None}
    };
    let name = match color.search_attribute_by_name("name"){
        Some(a_name) => a_name,
        None => {eprintln!("Not able to find 'name' attribute in this Node! {color}"); return None}
    };

    let Some(c) = parse_float_attribute(&color, "c") else {
        eprintln!("Not able to find 'c' attribute in this Node! {color}"); return None
    };
    let Some(m) = parse_float_attribute(&color, "m") else {
        eprintln!("Not able to find 'm' attribute in this Node! {color}"); return None
    };
    let Some(y) = parse_float_attribute(&color, "y") else {
        eprintln!("Not able to find 'y' attribute in this Node! {color}"); return None
    };
    let Some(k) = parse_float_attribute(&color, "k") else {
        eprintln!("Not able to find 'k' attribute in this Node! {color}"); return None
    };
    let Some(opacity) = parse_float_attribute(&color, "opacity") else {
        eprintln!("Not able to find 'opacity' attribute in this Node! {color}"); return None
    };

    Some(ColorAttributes {
        priority,
        name,
        cmyk: Cmyk { c, m, y, k },
        opacity,
    })
}
fn parse_cmyk(cmyk: Rc<Node>) -> Option<(Method, Option<Cmyk>)> {
    let Some(method) = parse_method(&cmyk) else {
        return None
    };
    let Some(c) = parse_float_attribute(&cmyk, "c") else {
        eprintln!("Not able to find 'c' attribute in this Node! {cmyk}"); return Some((method, None))
    };
    let Some(m) = parse_float_attribute(&cmyk, "m") else {
        eprintln!("Not able to find 'm' attribute in this Node! {cmyk}"); return Some((method, None))
    };
    let Some(y) = parse_float_attribute(&cmyk, "y") else {
        eprintln!("Not able to find 'y' attribute in this Node! {cmyk}"); return Some((method, None))
    };
    let Some(k) = parse_float_attribute(&cmyk, "k") else {
        eprintln!("Not able to find 'k' attribute in this Node! {cmyk}"); return Some((method, None))
    };
    Some((method, Some(Cmyk{c, m, y, k})))
}
fn parse_rgb(rgb: Rc<Node>) -> Option<(Method, Rgb)> {
    let Some(method) = parse_method(&rgb) else {
        return None
    };
    let Some(r) = parse_u8_attribute(&rgb, "r") else {
        eprintln!("Not able to find 'r' attribute in this Node! {rgb}"); return None
    };
    let Some(g) = parse_u8_attribute(&rgb, "g") else {
        eprintln!("Not able to find 'g' attribute in this Node! {rgb}"); return None
    };
    let Some(b) = parse_u8_attribute(&rgb, "b") else {
        eprintln!("Not able to find 'b' attribute in this Node! {rgb}"); return None
    };
    Some((method, Rgb{r, g, b}))
}
// fn parse_spotcolors

// Other helper structs
struct Cmyk{
    c: f32,
    m: f32,
    y: f32,
    k: f32,
}

struct Rgb{
    r: u8,
    g: u8,
    b: u8,
}

enum Method {
    Custom,
    SpotColor,
    Rgb,
}

struct ColorAttributes{
    priority: u32,
    name: String,
    cmyk: Cmyk,
    opacity: f32,
}