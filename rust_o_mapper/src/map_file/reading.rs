//! This file contains:
//! 1) Definition and Implementation of the Node Struct

use std::fmt;
use std::cell::RefCell;
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;
use quick_xml::events::Event;
use quick_xml::Reader;

/// A Node is the atomic part of the RAM representation of a .omap file.
/// A Node is equivalent to an XML Element.
/// The common anchestor of all nodes in a .omap file is the "<map>..</map> " element.
pub struct Node {
    /// The name of the Node
    pub name : String,
    /// The attributes of a Node are the attributes of XML element
    /// for example "alpha" and "beta" are the attributes of the following element:
    /// <map "alpha"=300, "beta"="rucola"></map>
    /// that in Node will become:
    /// attributes = [("alpha", "300"), ("beta", "rucola")]
    pub attributes : Vec<(String, String)>,
    /// The children of a Node are stored in a Vector of Nodes.
    pub children : RefCell<Vec<Rc<Node>>>,
    /// The indentation level store how deep we are in hierarchy.
    /// <map><children1></children1></map>
    pub indentation_level : i16,
    /// Th inner text store as a string the content of an xml element that are not other elements
    /// <map>This is stored as inner text!</map>
    pub inner_text: RefCell<Option<String>>,
}

impl Node {
    pub fn new(
        name : String,
        attributes : Vec<(String, String)>,
        indentation_level : i16,
    ) -> Rc<Self> {
        Rc::new(
            Node {
                name,
                attributes,
                children: RefCell::new(vec![]),
                indentation_level,
                inner_text: RefCell::new(None),
            }
        )
    }

    pub fn node_from_file(file_path: &str) -> Option<Rc<Self>> {
        // (1) Let's read the file
        let mut reader = match Reader::from_file(file_path){
            Ok(a_reader) => a_reader,
            Err(e) => {
                eprintln!("Error while trying to read {file_path}:\n\t{e}");
                return None;
            }
        };
        // (2) Remove white space before and after text!
        reader.config_mut().trim_text(true);

        // (3) Let's create a new node
        let bigger_ancestor = Node::new(
            "map_node".to_string(),
            vec![("path".to_string(), file_path.to_string())],
            0
        );

        // (4) Let's start populating the Node reading the xml
        bigger_ancestor.continue_xml_exploration(& mut reader);
        Some(bigger_ancestor)
    }

    pub fn set_inner_text(&self, inner_text : String){
        *self.inner_text.borrow_mut() = Some(inner_text);
    }

    fn continue_xml_exploration(self: & Rc<Self>, reader : &mut Reader<BufReader<File>>){
        let mut buf = Vec::new();
        loop {
            let event = match reader.read_event_into(&mut buf) {
                Ok(event) => event,
                Err(e) => {
                    eprintln!("Error while reading XML event: {e}\nStopping prematurally to read inside {}!", self.name);
                    break;
                }
            };
            match event {
                Event::Eof => break,
                // <tag attr="value">
                Event::Start(e) => {
                    self.children.borrow_mut()
                        .push(
                            Node::explore_a_start(
                                &e,
                                self.indentation_level,
                            )
                        );
                    let child = self.children.borrow().last().unwrap().clone();
                    child.continue_xml_exploration(reader);
                }
                // <tag attr="value" />
                Event::Empty(e) => {
                    self.children.borrow_mut()
                        .push(
                            Node::explore_a_start(
                                &e,
                                self.indentation_level,
                            )
                        );
                }
                // <tag>This is Text!</tag>
                Event::Text(e) => {
                    let inner_text = e.unescape().unwrap_or_else(|_| {
                        std::str::from_utf8(e.as_ref()).unwrap_or("Non UTF8 Text").into()
                    });
                    self.set_inner_text(inner_text.to_string());
                },
                // </tag>
                Event::End(e) => {
                    let name = std::str::from_utf8(e.name().into_inner()).unwrap_or("Non UTF8 Name");
                    if name != self.name {
                        eprintln!("Unexpected End event with a name different from the last start!\nStopping prematurally to read inside {}!", self.name);
                    }
                    break;
                }
                _ => (),
            }
            buf.clear();
        }
    }

    fn explore_a_start(e : & quick_xml::events::BytesStart,
                       last_indentation : i16,
                    ) -> Rc<Node> {
        let name = String::from(std::str::from_utf8(e.name().into_inner()).unwrap_or("Non UTF8 Name"));
        let mut attributes: Vec<(String, String)> = Vec::new();
        for attr_result in e.attributes() {
            match attr_result {
                Ok(attr) => {
                    let attribute_name = String::from(std::str::from_utf8(attr.key.into_inner()).unwrap_or("Non UTF8 Name"));
                    let attribute_value = String::from_utf8(attr.value.to_vec()).unwrap_or(String::from("Non UTF8 Name"));
                    attributes.push((attribute_name, attribute_value));
                },
                Err(error) => {
                    eprintln!("Error reading an attribute: {error}");
                }
            }
        }
        Node::new(
            name,
            attributes,
            last_indentation+1,
        )
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
        for child in self.children.borrow().iter(){
            if child.name == name_to_search {
                return Some(child.clone());
            }else if child.name == "barrier" { // Barrier is a special child that is treated as transparent while reading into childrens!
                if let Some(a_matchig_child) = child.search_child_by_name(name_to_search){
                    return Some(a_matchig_child);
                }
            }
        }
        None
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indentation = "  ".repeat(self.indentation_level as usize);
        f.write_str(&indentation)?;
        f.write_str(&self.name)?;
        f.write_str(" (")?;
        for (name, value) in &self.attributes {
            write!(f, "{}:\"{}\", ", name, value)?;
        }
        f.write_str(") {")?;
        if let Some(a_text) = self.inner_text.borrow().as_ref() {
            f.write_str(a_text)?;
        }
        f.write_str("}\n")?;
        for child in self.children.borrow().iter() {
            write!(f, "{}", child)?;
        }
        Ok(())
    }
}

