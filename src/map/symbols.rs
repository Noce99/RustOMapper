use std::rc::Rc;
use crate::map_file::reading::Node;

mod punctual;
mod linear;
mod area;
mod text;

pub trait Symbol{
    fn render(&self);
    fn show(&self) -> String;
}

pub struct SymbolsBag {
    // There we are creating a Vector of Box.
    // A Box is simply a container that store the data on the heap instead that on the stack, when
    // the owner change just the pointer in the stack is copied and not the real data on the heap.
    // dyn Symbol means that inside the Boxes we can have any struct that implement the Symbol trait
    bag: Vec<Box<dyn Symbol>>
}

impl SymbolsBag {
    pub fn new() -> Self {
        Self { bag: Vec::new() }
    }
    pub fn insert(&mut self, symbol: Box<dyn Symbol>) {
        self.bag.push(symbol);
    }
    pub fn len(&self) -> usize {
        self.bag.len()
    }

    pub fn symbols_from_a_node(symbols_node: Rc<Node>)-> Option<Self>{
        let mut bag: Vec<Box<dyn Symbol>> = Vec::new();
        for child in symbols_node.children.borrow().iter() {
            if child.name != "symbol"{
                continue;
            }
            let type_attribute_option = child.search_attribute_by_name("type");
            let the_type: String;
            match type_attribute_option{
                Some(a_type) => {the_type = a_type}
                None => {println!("I was not able to fine the 'type' attribute in a symbol node? WTF?"); return None}
            }
            let id_attribute_option = child.search_attribute_by_name("id");
            let the_id: u32;
            match id_attribute_option{
                Some(an_id) => {the_id = an_id.parse().unwrap()}
                None => {println!("I was not able to fine the 'id' attribute in a symbol node? WTF?"); return None}
            }
            let code_attribute_option = child.search_attribute_by_name("code");
            let the_code: String;
            match code_attribute_option{
                Some(a_code) => {the_code = a_code}
                None => {println!("I was not able to fine the 'code' attribute in a symbol node? WTF?"); return None}
            }
            let name_attribute_option = child.search_attribute_by_name("name");
            let the_name: String;
            match name_attribute_option{
                Some(a_name) => {the_name = a_name}
                None => {println!("I was not able to fine the 'name' attribute in a symbol node? WTF?"); return None}
            }
            let description_attribute_option = child.search_attribute_by_name("description");
            let the_description: String;
            match description_attribute_option{
                Some(a_description) => {the_description = a_description}
                None => {the_description = String::new()}
            }
            if the_type == "2"{
                bag.push(Box::new(linear::LinearSymbol::new(the_id, the_code, the_name, the_description)));
            }else if the_type == "4" || the_type == "16" {
                bag.push(Box::new(area::AreaSymbol::new(the_id, the_code, the_name, the_description)));
            }else if the_type == "1" {
                bag.push(Box::new(punctual::PunctualSymbol::new(the_id, the_code, the_name, the_description)));
            }else if the_type == "8" {
                bag.push(Box::new(text::TextSymbol::new(the_id, the_code, the_name, the_description)));
            }else{
                let name_option = child.search_attribute_by_name("name");
                let name: String;
                match name_option{
                    Some(a_name) => {name = a_name}
                    None => {println!("I was not able to fine the type attribute in a symbol node? WTF?"); return None}
                }
                println!("Find out a strange type for symbol ({}) [type={}]", name, the_type);
            }
        }
        Some(SymbolsBag{
            bag
        })
    }

    pub fn show(&self){
        for symbol in &self.bag{
            println!("{}", symbol.show())
        }
    }
}