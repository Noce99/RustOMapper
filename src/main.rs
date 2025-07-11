mod map_file;
mod map;

use crate::map_file::reading::read_o_mapper_file;
use crate::map_file::explorer::explore_children;


fn main() {
    let a_map = read_o_mapper_file("Maps/BOLOLECCHIO_ISSPROM_2024_10_25.omap");

    // print!("{}", a_map);
    explore_children(a_map);

    /*
    let struct_types = a_map.get_struct_types();
    for struct_type in &struct_types{
        println!("{}", struct_type);
    }
    */
}
