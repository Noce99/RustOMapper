use std::fmt;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::cell::RefCell;
use std::fs::File;
use std::io::BufReader;
use std::rc::{Rc, Weak};
use std::time::Instant;

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
    pub fn new(
        name : String,
        attributes : Vec<(String, String)>,
        indentation_level : i16,
        parent_option: Option<Weak<Node>>
    ) -> Rc<Self> {
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
    pub fn node_from_file(file_path: &str) -> Option<Rc<Self>> {
        // (1) Let's read the file
        println!("I'm going to open: {file_path}");
        let start_file_reading_time = Instant::now();
        let reader_result = Reader::from_file(file_path);
        let mut reader: Reader<BufReader<File>>;
        match reader_result{
            Ok(a_reader) => {reader = a_reader;}
            Err(e) => {
                eprintln!("Error while trying to read {file_path}:\n\t{e}");
                return None;
            }
        }
        // (2) Let's remove unwanted white spaces
        reader.config_mut().trim_text(true); // Remove white space!

        // (3) Let's create a new node
        let bigger_ancestor = Rc::new(Self{
            name: "map_node".to_string(),
            attributes: vec![("path".to_string(), file_path.to_string())],
            children: RefCell::new(vec![]),
            parent: RefCell::new(Default::default()),
            indentation_level: 0,
            inner_text: RefCell::new(None),
        });
        // (4) Let's start populating the Node reading the xml
        bigger_ancestor.continue_xml_exploration(& mut reader);
        // (5) Let's debug the time needed to read the xml
        let file_reading_time = start_file_reading_time.elapsed();
        let file_reading_time_s = file_reading_time.as_secs();
        let file_reading_time_ms = file_reading_time.subsec_millis();
        println!("Built the node tree from the xml file in {file_reading_time_s} s and \
         {file_reading_time_ms} ms.");
        Some(bigger_ancestor)
    }
    pub fn set_inner_text(&self, inner_text : String){
        *self.inner_text.borrow_mut() = Some(inner_text);
    }
    fn continue_xml_exploration(self: & Rc<Self>, reader : &mut Reader<BufReader<File>>){
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf).unwrap() {
                Event::Eof => break,
                Event::Start(e) => {
                    self.children.borrow_mut()
                        .push(
                            self.explore_a_start(
                                &e,
                                self.indentation_level,
                                Some(Rc::downgrade(self))
                            )
                        );
                    buf.clear();
                    let last_child_index = self.children.borrow().len() - 1;
                    self.children.borrow_mut()[last_child_index].continue_xml_exploration(reader);
                }
                Event::Empty(e) => {
                    self.children.borrow_mut()
                        .push(
                            self.explore_a_start(
                                &e,
                                self.indentation_level,
                                Some(Rc::downgrade(self))
                            )
                        );
                    buf.clear();
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

    fn explore_a_start(self: & Rc<Self>,
                       e : & quick_xml::events::BytesStart,
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

    pub fn get_name(& self) -> String{
        self.name.clone()
    }

    pub fn get_struct_types(& self) -> Vec<StructTypes>{
        let mut struct_types: Vec<StructTypes> = Vec::new();
        update_or_create_new_struct_type(& self, &mut struct_types);
        struct_types
    }

    pub fn search_attribute_by_name(& self, name_to_search: &str) -> Option<String>{
        for (name, value) in &self.attributes{
            if name == name_to_search{
                return Some(value.clone());
            }
        }
        None
    }

    pub fn search_child_by_name(& self, name_to_search: &str) -> Option<Rc<Node>>{
        let mut result_of_the_search: Option<Rc<Node>> = None;
        let mut counter: usize = 0;
        for child in self.children.borrow().iter(){
            if child.name == name_to_search {
                result_of_the_search = Some(self.children.borrow()[counter].clone());
            }else if child.name == "barrier" {
                let result_of_searching_inside_a_barrier = child.search_child_by_name(name_to_search);
                match result_of_searching_inside_a_barrier {
                    None => {}
                    Some(a_node) => {result_of_the_search = Some(a_node)}
                }
            }
            counter += 1;
        }
        result_of_the_search
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

