mod map_file;
mod map;

use crate::map_file::reading::read_o_mapper_file;
use crate::map_file::explorer::explore_children;
use crate::map::colors::{Color, ColorsBag};
use crate::map::Map;


fn main() {
    let a_node = read_o_mapper_file("Maps/BOLOLECCHIO_ISSPROM_2024_10_25.omap");

    // print!("{}", a_node);
    //explore_children(a_node.clone());
    let a_map: Map;
    let a_map_option = Map::new(a_node.clone());
    match a_map_option {
        None => {panic!("I was not able to create a map object!")},
        Some(aa_map) => {a_map = aa_map}
    }
    // a_map.colors.show()
    a_map.symbols.show()


    /*
    let struct_types = a_node.get_struct_types();
    for struct_type in &struct_types{
        println!("{}", struct_type);
    }
    */
}
