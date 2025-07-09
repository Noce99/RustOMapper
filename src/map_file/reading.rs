use std::fmt;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::map::symbols::SymbolsBag;

pub struct Node {
    pub name : String,
    pub attributes : Vec<(String, String)>,
    pub children : RefCell<Vec<Rc<Node>>>,
    pub parent: RefCell<Weak<Node>>,
    pub indentation_level : i16,
    pub inner_text: RefCell<Option<String>>,
}

impl Node {
    pub fn new(name : String, attributes : Vec<(String, String)>, indentation_level : i16, parent_option: Option<Weak<Node>>) -> Rc<Node> {
        let parent: Weak<Node>;
        match parent_option{
            None => {parent = Weak::new()}
            Some(a_parent) => {parent = a_parent}
        }
        Rc::new(
            Node {
                name,
                attributes,
                parent: RefCell::new(parent),
                children: RefCell::new(vec![]),
                indentation_level,
                inner_text: RefCell::new(None),
            }
        )
    }
    pub fn set_inner_text(&self, inner_text : String){
        *self.inner_text.borrow_mut() = Some(inner_text);
    }
    pub fn continue_xml_exploration(self: & Rc<Self>, reader : &mut Reader<std::io::BufReader<std::fs::File>>){
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf).unwrap() {
                Event::Eof => break,
                Event::Start(e) => {
                    self.children.borrow_mut().push(explore_a_start(e, self.indentation_level, Some(Rc::downgrade(self))));
                    buf.clear();
                    let last_child_index = self.children.borrow().len() - 1;
                    self.children.borrow_mut()[last_child_index].continue_xml_exploration(reader);
                }
                Event::Text(e) => {
                    let inner_text = e.unescape().unwrap();
                    self.set_inner_text(inner_text.to_string());
                },
                Event::End(e) => {
                    let name = std::str::from_utf8(e.name().into_inner()).unwrap();
                    if name != self.name {
                        panic!("Unexpected End event with a name different from the last start!");
                    }
                    break;
                }
                _ => (),
            }
            buf.clear();
        }
    }
    pub fn get_name(& self) -> String{
        self.name.clone()
    }

    pub fn get_struct_types(& self) -> Vec<StructTypes>{
        let mut struct_types: Vec<StructTypes> = Vec::new();
        update_or_create_new_struct_type(& self, &mut struct_types);
        struct_types
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut indentation: String = String::new();
        for _i in 0..self.indentation_level{
            indentation.push_str("  ");
        }
        let mut output: String = String::new();
        output.push_str(&indentation);
        output.push_str(&self.name);
        output.push(' ');
        output.push('(');
        for (name, value) in &self.attributes{
            output.push_str(&format!("{}:\"{}\", ", name, value));
        }
        output.push(')');
        output.push(' ');
        output.push('{');
        match self.inner_text.borrow_mut().as_ref() {
            None => {}
            Some(a_text) => {output.push_str(a_text);}
        }
        output.push('}');
        output.push('\n');
        for child in self.children.borrow().iter(){
            output.push_str(&format!("{}", child));
        }
        write!(f, "{output}")
    }
}

fn explore_a_start(e : quick_xml::events::BytesStart,
                   last_indentation : i16,
                   parent_option: Option<Weak<Node>>) -> Rc<Node> {
    let name = String::from(std::str::from_utf8(e.name().into_inner()).unwrap());
    let mut attributes: Vec<(String, String)> = Vec::new();
    for attr in e.attributes() {
        let real_attr = attr.unwrap();
        let attribute_name = String::from(std::str::from_utf8(real_attr.key.into_inner()).unwrap());
        let attribute_value = String::from_utf8(real_attr.value.to_vec()).unwrap();
        attributes.push((attribute_name, attribute_value));
    }
    Node::new(
        name,
        attributes,
        last_indentation+1,
        parent_option,
    )
}

#[derive(Debug)]
pub struct StructTypes{
    name : String,
    attributes : Vec<String>,
    attribute_values : Vec<Vec<String>>,
}

impl fmt::Display for StructTypes{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output: String = String::new();
        output.push_str(&self.name);
        output.push('\n');
        for i in 0..self.attributes.len(){
            output.push('\t');
            output.push_str(&self.attributes[i]);
            output.push_str(" : [");
            for ii in 0..self.attribute_values[i].len(){
                output.push_str(&format!("{}; ", &self.attribute_values[i][ii]));
            }
            output.push_str("]\n");
        }
        write!(f, "{output}")
    }
}

fn update_or_create_new_struct_type(an_object: &Node, all_struct_types: &mut Vec<StructTypes>){
    let mut found_struct = false;
    // (1) Let's start searching for an_object inside all_struct_types
    for i in 0..all_struct_types.len(){
        if an_object.name == all_struct_types[i].name{
            found_struct = true;
            // (2)  If we found that the object already exist inside all_struct_types, we have to
            //      compare both the attribute names and values.
            //      Let's iterate over all the attributes of the object that we are searching.
            for (attribute_name, attribute_value) in an_object.attributes.iter(){
                let mut found_attribute = false;
                // (3)  Let's compare all the attribute of our object with all the struct that
                //      matched attributes.
                for (ii, struct_attribute_name) in all_struct_types[i].attributes.iter().enumerate() {
                    // (4)  If the struct that matched and our object both have the same attribute
                    //      name. Let's iterate over that attribute values.
                    if attribute_name == struct_attribute_name{
                        found_attribute = true;
                        let mut found_values = false;
                        // (5) Let's compare all the attribute values of the matched struct with
                        //     the value of the attribute that matched.
                        for struct_attribute_values in all_struct_types[i].attribute_values[ii].iter(){
                            // (6) If the value is already there we do not need to add it.
                            if attribute_value == struct_attribute_values{
                                found_values = true;
                                break;
                            }
                        }
                        // (7) If it's not there we should add it.
                        if !found_values{
                            all_struct_types[i].attribute_values[ii].push(String::from(attribute_value));
                        }
                        break;
                    }
                }
                // (8)  If the attribute name was not present in the matched struct attributes we
                //      should add it.
                if !found_attribute{
                    all_struct_types[i].attributes.push(String::from(attribute_name));
                    all_struct_types[i].attribute_values.push(Vec::from([attribute_value.clone()]));
                }
            }
            break;
        }
    }
    // (9) If our object didn't match any struct we should add a new struct in our list!
    if !found_struct{
        let mut attributes: Vec<String> = Vec::new();
        let mut attribute_values: Vec<Vec<String>> = Vec::new();
        for (attribute_name, attribute_value) in an_object.attributes.iter() {
            attributes.push(String::from(attribute_name));
            attribute_values.push(Vec::from([attribute_value.clone()]));
        }
        all_struct_types.push(StructTypes{
            name: an_object.get_name(),
            attributes,
            attribute_values,
        })
    }
    // (10) Let's check all the children of our object!
    for child in an_object.children.borrow().iter(){
        update_or_create_new_struct_type(child, all_struct_types);
    }
}

pub fn read_o_mapper_file(file_path: &str) -> Rc<Node> {
    // (1) Let's open the file
    let mut reader = Reader::from_file(file_path)
        .expect("Failed to open the map file!");
    // (2) Let's remove unwanted white spaces
    reader.config_mut().trim_text(true); // Remove white space!

    // (3) Let's start reading events!
    let mut buf = Vec::new();
    let mut bigger_ancestor;
    loop {
        let event = reader.read_event_into(& mut buf)
            .expect("Error while reading an XML event! :-(");
        match event {
            Event::Start(e) => {
                bigger_ancestor = explore_a_start(e, - 1, None);
                bigger_ancestor.continue_xml_exploration(& mut reader);
                break
            }
            _ => continue,
        };
    }
    buf.clear();
    bigger_ancestor
    /*
    I want this function to return a SymbolsBag!
     */
}

