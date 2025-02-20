mod map_file;
mod map;

use crate::map_file::reading::read_o_mapper_file;

fn main() {
    let a_map = read_o_mapper_file("/");
}
