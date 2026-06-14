use std::any::Any;
use std::rc::Rc;
use crate::map_file::reading::Node;
use std::collections::HashMap;
use crate::map::symbols::geometric_shape::GeometricShape;


pub mod punctual;
pub mod linear;
pub mod area;
pub mod text;
pub mod geometric_shape;
pub mod svg;

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
        let symbol_type = match node.search_attribute_by_name("type"){
            Some(a_type) => a_type,
            None => {eprintln!("I was not able to fine the 'type' attribute in a symbol node? WTF?"); return None}
        };
        let id = match node.search_attribute_by_name("id"){
            Some(an_id) => an_id.parse().unwrap(),
            None => 0
        };
        let code = match node.search_attribute_by_name("code"){
            Some(a_code) => a_code,
            None => {eprintln!("I was not able to fine the 'code' attribute in a symbol node? WTF?"); return None}
        };
        let name = match node.search_attribute_by_name("name"){
            Some(a_name) => a_name,
            None => String::new(),
        };
        let description = match node.search_attribute_by_name("description") {
            Some(a_description) => a_description,
            None => String::new(),
        };
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
        let mut bag: HashMap<i32, Box<dyn Symbol>> = HashMap::new();
        for child in symbols_node.children.borrow().iter() {
            if child.name != "symbol"{
                continue;
            }
            // We create a Symbol Common to get the basic information that I need for all symbol type
            let basic_symbol = match SymbolCommon::new_from_node(child) {
                Some(a_symbol) => a_symbol,
                None => {eprintln!("I was not able to create a SymbolCommon from a Symbol Node?"); return None}
            };
            // Based on the symbol type I create (and add to the bag) different symbols
            if basic_symbol.symbol_type == "2"{
                // Linear Symbol
                let linear_symbol = match linear::LinearSymbol::symbol_from_a_node(&basic_symbol, child){
                    Some(a_linear_symbol) => a_linear_symbol,
                    None => {eprintln!("I was not able to create a LinearSymbol from a Symbol Node. [name = {}, type = {}]", basic_symbol.name.clone(), basic_symbol.symbol_type.clone()); return None}
                };
                bag.insert(linear_symbol.get_id(), linear_symbol);
            }else if basic_symbol.symbol_type == "4" {
                // Area Symbol
                let area_symbol = match area::AreaSymbol::symbol_from_a_node(&basic_symbol, child){
                    Some(an_area_symbol) => an_area_symbol,
                    None => {eprintln!("I was not able to create a AreaSymbol from a Symbol Node. [name = {}, type = {}]", basic_symbol.name.clone(), basic_symbol.symbol_type.clone()); return None}
                };
                bag.insert(area_symbol.get_id(), area_symbol);
            }else if basic_symbol.symbol_type == "16" {
                // Combined Symbol
            }else if basic_symbol.symbol_type == "1" {
                // Punctual Symbol
                let punctual_symbol = match punctual::PunctualSymbol::symbol_from_a_node(&basic_symbol, child){
                    Some(a_punctual_symbol) => a_punctual_symbol,
                    None => {eprintln!("I was not able to create a PunctualSymbol from a Symbol Node. [name = {}, type = {}]", basic_symbol.name.clone(), basic_symbol.symbol_type.clone()); return None}
                };
                bag.insert(punctual_symbol.get_id(), punctual_symbol);
            }else if basic_symbol.symbol_type == "8" {
                // Text Symbol
                let text_symbol = match text::TextSymbol::symbol_from_a_node(&basic_symbol, child){
                    Some(a_text_symbol) => a_text_symbol,
                    None => {eprintln!("I was not able to create a TextSymbol from a Symbol Node. [name = {}, type = {}]", basic_symbol.name.clone(), basic_symbol.symbol_type.clone()); return None}
                };
                bag.insert(text_symbol.get_id(), text_symbol);
            }else{
                eprintln!("Find out a strange type for symbol ({}) [type={}]", basic_symbol.name, basic_symbol.symbol_type);
            }
        }
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