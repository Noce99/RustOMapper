use std::any::Any;
use std::rc::Rc;
use std::time::Instant;
use crate::map_file::reading::Node;
use std::collections::HashMap;
use crate::map::symbols::geometric_shape::GeometricShape;


pub mod punctual;
pub mod linear;
pub mod area;
pub mod text;
pub mod geometric_shape;

#[derive(Debug)]
pub struct SymbolCommon{
    symbol_type: String,
    id: i32,
    code: String,
    name: String,
    description: String,
}

impl SymbolCommon{
    fn new_from_node(node: &Rc<Node>) -> Option<Self> {
        let type_attribute_option = node.search_attribute_by_name("type");
        let symbol_type: String;
        match type_attribute_option{
            Some(a_type) => {symbol_type = a_type}
            None => {println!("I was not able to fine the 'type' attribute in a symbol node? WTF?"); return None}
        }
        let id_attribute_option = node.search_attribute_by_name("id");
        let id: i32;
        match id_attribute_option{
            Some(an_id) => {id = an_id.parse().unwrap()}
            None => {id = 0;}
        }
        let code_attribute_option = node.search_attribute_by_name("code");
        let code: String;
        match code_attribute_option{
            Some(a_code) => {code = a_code}
            None => {println!("I was not able to fine the 'code' attribute in a symbol node? WTF?"); return None}
        }
        let name_attribute_option = node.search_attribute_by_name("name");
        let name: String;
        match name_attribute_option{
            Some(a_name) => {name = a_name}
            None => {name = String::new()}
        }
        let description_attribute_option = node.search_attribute_by_name("description");
        let description: String;
        match description_attribute_option {
            Some(a_description) => {description = a_description}
            None => { description = String::new()}
        }
        Some(Self{
            symbol_type,
            id,
            code,
            name,
            description,
        })
    }
}

pub trait Symbol: Any{
    // fn render(&self);
    // fn show(&self) -> String;

    fn get_id(&self) -> i32;

    fn get_symbol_type(&self) -> String;

    fn get_geometric_shapes(&self) -> & Vec<Rc<dyn GeometricShape>>;
    // fn get_name(&self) -> &str;
}

pub struct SymbolsBag {
    // There we are creating a Vector of Box.
    // A Box is simply a container that store the data on the heap instead that on the stack, when
    // the owner change just the pointer in the stack is copied and not the real data on the heap.
    // dyn Symbol means that inside the Boxes we can have any struct that implement the Symbol trait
    pub bag: HashMap<i32, Box<dyn Symbol>>
}

impl SymbolsBag {
    // pub fn new() -> Self {
    //     Self { bag: HashMap::new() }
    // }
    // pub fn insert(&mut self, symbol: Box<dyn Symbol>) {
    //     self.bag.insert(symbol.get_id(), symbol);
    // }
    pub fn len(&self) -> usize {
        self.bag.len()
    }

    pub fn symbols_from_a_node(symbols_node: Rc<Node>)-> Option<Self>{
        println!("I'm going to create a Symbols Bag!");
        let start_symbols_bag_creation_time = Instant::now();
        let mut bag: HashMap<i32, Box<dyn Symbol>> = HashMap::new();
        for child in symbols_node.children.borrow().iter() {
            if child.name != "symbol"{
                continue;
            }
            // We create a Symbol Common to get the basic information that I need for all symbol type
            let a_symbol_option = SymbolCommon::new_from_node(child);
            let basic_symbol;
            match a_symbol_option {
                Some(a_symbol) => {basic_symbol = a_symbol}
                None => {eprintln!("I was not able to create a SymbolCommon from a Symbol Node?"); return None}
            }
            // Based on the symbol type I create (and add to the bag) different symbols
            if basic_symbol.symbol_type == "2"{
                // Linear Symbol
                let linear_symbol_option = linear::LinearSymbol::symbol_from_a_node(&basic_symbol, child);
                let linear_symbol;
                match linear_symbol_option{
                    Some(a_linear_symbol) => {linear_symbol = a_linear_symbol;}
                    None => {eprintln!("I was not able to create a LinearSymbol from a Symbol Node. [name = {}, type = {}]", basic_symbol.name.clone(), basic_symbol.symbol_type.clone()); return None}
                }
                bag.insert(linear_symbol.get_id(), linear_symbol);
            }else if basic_symbol.symbol_type == "4" || basic_symbol.symbol_type == "16" {
                // Area Symbol
                let area_symbol_option = area::AreaSymbol::symbol_from_a_node(&basic_symbol, child);
                let area_symbol;
                match area_symbol_option{
                    Some(an_area_symbol) => {area_symbol = an_area_symbol;}
                    None => {eprintln!("I was not able to create a AreaSymbol from a Symbol Node. [name = {}, type = {}]", basic_symbol.name.clone(), basic_symbol.symbol_type.clone()); return None}
                }
                bag.insert(area_symbol.get_id(), area_symbol);
            }else if basic_symbol.symbol_type == "1" {
                // Punctual Symbol
                let punctual_symbol_option = punctual::PunctualSymbol::symbol_from_a_node(&basic_symbol, child);
                let punctual_symbol;
                match punctual_symbol_option{
                    Some(a_punctual_symbol) => {punctual_symbol = a_punctual_symbol;}
                    None => {eprintln!("I was not able to create a PunctualSymbol from a Symbol Node. [name = {}, type = {}]", basic_symbol.name.clone(), basic_symbol.symbol_type.clone()); return None}
                }
                bag.insert(punctual_symbol.get_id(), punctual_symbol);
            }else if basic_symbol.symbol_type == "8" {
                // Text Symbol
                let text_symbol_option = text::TextSymbol::symbol_from_a_node(&basic_symbol, child);
                let text_symbol;
                match text_symbol_option{
                    Some(a_text_symbol) => {text_symbol = a_text_symbol;}
                    None => {eprintln!("I was not able to create a TextSymbol from a Symbol Node. [name = {}, type = {}]", basic_symbol.name.clone(), basic_symbol.symbol_type.clone()); return None}
                }
                bag.insert(text_symbol.get_id(), text_symbol);
            }else{
                println!("Find out a strange type for symbol ({}) [type={}]", basic_symbol.name, basic_symbol.symbol_type);
            }
        }
        let symbols_bag_time = start_symbols_bag_creation_time.elapsed();
        let symbols_bag_time_s = symbols_bag_time.as_secs();
        let symbols_bag_time_ms = symbols_bag_time.subsec_millis();
        println!("Created the Symbols Bag in {symbols_bag_time_s} s and \
         {symbols_bag_time_ms} ms.");
        Some(SymbolsBag{
            bag
        })
    }

    // pub fn show(&self){
    //     for (id, symbol) in &self.bag{
    //         println!("{}", symbol.show())
    //     }
    // }

    pub fn symbol_by_id(&self, id: i32) -> Option<&Box<dyn Symbol>>{
        let to_return = self.bag.get(&id);
        match to_return{
            Some(_) => {}
            None => {eprintln!("I was not able to find symbol {0} in the Symbols Bag but a reference was found in an element!?", id)}
        }
        to_return
    }
}