use crate::map::colors::ColorsBag;
use crate::map::symbols::SymbolsBag;
use crate::map::elements::ElementsBag;
use crate::map_file::reading::Node;
use crate::map::symbols::geometric_shape::GeometricShapesBag;

use prettytable::Table;
use std::time::Instant;

pub mod symbols;
pub mod colors;
pub mod yaml_encoding;
pub mod elements;

pub struct Map{
    pub colors: ColorsBag,
    pub symbols: SymbolsBag,
    pub elements: ElementsBag,
    // pub map_node: Rc<Node>,
    pub geometric_shapes: GeometricShapesBag,
}

impl Map {
    pub fn new(file_path: &str) -> Option<Map> {
        let mut map_creation_stats_table = Table::new();
        map_creation_stats_table.set_titles(row![iH2->file_path]);
        map_creation_stats_table.add_row(row![c->"Work", c->"Time Needed (s)"]);

        let mut total_time : u64 = 0;
        let start_time = Instant::now();
        
        let node = match Node::node_from_file(file_path){
            Some(node) => node,
            None => {eprintln!("I was not able to read the map located in {}", file_path);return None;}
        };
        let map_node = match node.search_child_by_name("map") {
            Some(node) => node,
            None => {eprintln!("Not possible to find child named 'map' in the current Node!");return None;}
        };
        map_creation_stats_table.add_row(row![
            c->"XML Parsing",
            bc->stop_timer_and_get_formatted_time(start_time, &mut total_time),
        ]);

        // We get the COLORS
        let start_time = Instant::now();
        let colors_node = match map_node.search_child_by_name("colors") {
            Some(a_colors_node) => a_colors_node,
            None => {eprintln!("Not possible to find child named 'colors' in the current Node!"); return None}
        };

        let colors = match ColorsBag::new(colors_node.clone()) {
            Some(a_colors) => a_colors,
            None => {eprintln!("Not able to create a ColorsBag from a colors Node!"); return None}
        };
        map_creation_stats_table.add_row(row![
            c->"Colors Bag Creation",
            bc->stop_timer_and_get_formatted_time(start_time, &mut total_time),
        ]);

        // We get the SYMBOLS
        let start_time = Instant::now();
        let symbols_node = match map_node.search_child_by_name("symbols") {
            Some(a_symbols_node) => a_symbols_node,
            None => {eprintln!("Not possible to find a child named 'symbols' in the current Node!"); return None}
        };
        let symbols = match SymbolsBag::symbols_from_a_node(symbols_node.clone()){
            Some(a_symbols_bag) => a_symbols_bag,
            None => {eprintln!("Not possible to create a SymbolsBag from a Node!"); return None}
        };
        map_creation_stats_table.add_row(row![
            c->"Symbols Bag Creation",
            bc->stop_timer_and_get_formatted_time(start_time, &mut total_time),
        ]);

        // We get the elements
        let start_time = Instant::now();
        let parts = map_node.search_child_by_name("parts").unwrap();
        let elements = ElementsBag::elements_from_parts(parts);
        map_creation_stats_table.add_row(row![
            c->"Elements Bag Creation",
            bc->stop_timer_and_get_formatted_time(start_time, &mut total_time),
        ]);
        
        // We get Geometric Shapes
        let start_time = Instant::now();
        let geometric_shapes = GeometricShapesBag::from_elements_bag(&elements, &symbols);
        map_creation_stats_table.add_row(row![
            c->"Geometric Shapes Bag Creation",
            bc->stop_timer_and_get_formatted_time(start_time, &mut total_time),
        ]);

        // Let's print the table with all the times
        map_creation_stats_table.add_row(row![
            Frbc->"Total",
            bc->format_micros(total_time),
        ]);
        map_creation_stats_table.printstd();


        Some(Map {
            colors,
            symbols,
            elements,
            // map_node,
            geometric_shapes
        })
    }
}


fn format_micros(micros: u64) -> String{
    let seconds: u32 = (micros / 10u64.pow(6)) as u32;
    let remaining_us: u64 = micros % 10u64.pow(6);

    format!("{}.{:06}", seconds, remaining_us)
}

fn stop_timer_and_get_formatted_time(start_time: Instant, total_time: &mut u64) -> String{
    let elapsed_time = start_time.elapsed();

    let total_micros = elapsed_time.as_micros() as u64;

    *total_time += total_micros;

    format_micros(total_micros)
}