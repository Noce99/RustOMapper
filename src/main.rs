mod map_file;
mod map;

use std::process::exit;
use std::rc::Rc;
use crate::map_file::reading::Node;
use crate::map_file::explorer::explore_children;
use crate::map::colors::{Color, ColorsBag};
use crate::map::Map;


fn main() {
    let a_map_path = "Maps/BOLOLECCHIO_ISSPROM_2024_10_25.omap";
    let a_node_option = Node::node_from_file(a_map_path);
    let a_node;
    match a_node_option{
        None => {
            eprintln!("I was not able to read the map located in {a_map_path}");
            exit(1)}
        Some(b_node) => {a_node = b_node}
    }

    // print!("{}", a_node);
    //explore_children(a_node.clone());

    let a_map: Map;
    let a_map_option = Map::new(a_node);
    match a_map_option {
        None => {panic!("I was not able to create a map object!")},
        Some(aa_map) => {a_map = aa_map}
    }
    a_map.colors.show();
    a_map.symbols.show();

    /*
    let struct_types = a_node.get_struct_types();
    for struct_type in &struct_types{
        println!("{}", struct_type);
    }
    */
}
