use std::rc::Rc;
use crate::map::colors::ColorsBag;
use crate::map::symbols::SymbolsBag;
use crate::map_file::reading::Node;

pub(crate) mod symbols;
pub mod colors;

pub struct Map{
    pub colors: ColorsBag,
    pub symbols: SymbolsBag
}

impl Map {
    pub fn new(a_node: Rc<Node>) -> Option<Map> {
        // We get the COLORS
        let colors_node_option: Option<Rc<Node>> = a_node.search_child_by_name("colors");
        let colors_node: Rc<Node>;
        match colors_node_option {
            None => {println!("Not possible to fina child named colors in the current Node!"); return None}
            Some(a_colors_node) => {colors_node = a_colors_node}
        }
        let colors_option: Option<ColorsBag> = ColorsBag::new(colors_node.clone());
        let colors: ColorsBag;
        match colors_option {
            None => {println!("Not able to create a ColorsBag from a colors Node!"); return None}
            Some(a_colors) => {colors = a_colors}
        }
        // We get the SYMBOLS
        let symbols_node_option: Option<Rc<Node>> = a_node.search_child_by_name("symbols");
        let symbols_node: Rc<Node>;
        match symbols_node_option {
            None => {println!("Not possible to find a child named symbols in the current Node!"); return None}
            Some(a_symbols_node) => {symbols_node = a_symbols_node}
        }
        let symbols: SymbolsBag;
        let symbols_option: Option<SymbolsBag> = SymbolsBag::symbols_from_a_node(symbols_node.clone());
        match symbols_option {
            None => {println!("Not possible to create a SymbolsBag from a Node!"); return None}
            Some(a_symbols_bag) => {symbols = a_symbols_bag}
        }
        Some(Map {
            colors,
            symbols
        })
    }
}