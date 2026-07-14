use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

use crate::map::colors::ColorsBag;
use crate::map::elements::ElementsBag;
use crate::map::symbols::SymbolsBag;
use crate::map::symbols::geometric_shape::from_number_to_vec_of_nodes;

pub struct MapSVG{
    pub document : Document
}

impl MapSVG{
    pub fn from_elements_and_symbols(elements_bag : & ElementsBag, symbols : & SymbolsBag, colors : & ColorsBag) -> MapSVG{
        let mut a_document = Document::new();
        a_document = a_document.set("viewBox", (elements_bag.min_x.unwrap(), elements_bag.min_y.unwrap(), elements_bag.max_x.unwrap(), elements_bag.max_y.unwrap()));

        for element in & elements_bag.bag {
            let symbol = symbols.symbol_by_id(element.symbol);
            
            match symbol{
                Some(a_symbol) => {
                    for geometric_shape in a_symbol.get_geometric_shapes(){
                        if a_symbol.get_symbol_type() == "punctual"{
                            
                        }else if a_symbol.get_symbol_type() == "area"{
                            let color: String = colors.colors[&geometric_shape.get_color()].get_string_rgb();
                            let nodes: Vec<super::geometric_shape::Node> = from_number_to_vec_of_nodes(element.coordinates.clone());
                            if nodes.len() >= 1{
                                let mut data = Data::new().move_to((nodes[0].point.x, nodes[0].point.y));
                                for i in 0..(nodes.len()-1){
                                    if nodes[i].right_branch.as_ref().is_some() {
                                        assert!(nodes[i+1].left_branch.is_some(), "WTF, a Node with a NOT None right branch ad the consecutive node has a None left branch?");
                                        data = data.cubic_curve_to((
                                            nodes[i].right_branch.as_ref().unwrap().x,
                                            nodes[i].right_branch.as_ref().unwrap().y,
                                            nodes[i+1].left_branch.as_ref().unwrap().x,
                                            nodes[i+1].left_branch.as_ref().unwrap().y,
                                            nodes[i+1].point.x,
                                            nodes[i+1].point.y,
                                        ));
                                    }else{
                                        data = data.line_to((nodes[i+1].point.x, nodes[i+1].point.y));
                                    }
                                }
                                let path = Path::new()
                                    .set("fill", color)
                                    .set("d", data);
                                a_document = a_document.add(path);
                            }
                        }
                    }
                }
                None => {}
            }
        }
        svg::save("image.svg", &a_document).unwrap();
        MapSVG{document: a_document}
    }
}