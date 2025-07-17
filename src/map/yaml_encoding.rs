use std::any::{type_name, Any};
use serde::{Deserialize, Serialize, Serializer};
use crate::map::colors::ColorsBag;
use crate::map::Map;
use std::any::TypeId;
use crate::map::symbols::area::AreaSymbol;
use crate::map::symbols::geometric_shape::{Annulus, Area, Line};
use crate::map::symbols::linear::LinearSymbol;
use crate::map::symbols::punctual::PunctualSymbol;
use crate::map::symbols::text::TextSymbol;

#[derive(Serialize, Deserialize)]
struct ColorsYaml {
    num: usize,
    colors: Vec<ColorYaml>,
}

#[derive(Serialize, Deserialize)]
struct ColorYaml {
    priority: u32,
    name: String,
    r:  u8,
    g:  u8,
    b:  u8,
}

#[derive(Serialize, Deserialize)]
struct SymbolsYaml {
    num: usize,
    punctual_symbols: Vec<PunctualSymbolYaml>,
    linear_symbols: Vec<LinearSymbolYaml>,
    area_symbols: Vec<AreaSymbolYaml>,
    text_symbols: Vec<TextSymbolYaml>,
}

#[derive(Serialize, Deserialize)]
struct PunctualSymbolYaml {
    id: u32,
    code: String,
    name: String,
    description: String,
    annulus: Vec<AnnulusYaml>,
    lines: Vec<LineYaml>,
    areas: Vec<AreaYaml>,
    texts: Vec<TextSymbolYaml>,
}

#[derive(Serialize, Deserialize)]
struct AreaSymbolYaml {
}

#[derive(Serialize, Deserialize)]
struct TextSymbolYaml {
}

#[derive(Serialize, Deserialize)]
struct LinearSymbolYaml {
}

#[derive(Serialize, Deserialize)]
struct LineYaml {

}

#[derive(Serialize, Deserialize)]
struct AnnulusYaml {
    inner_radius: u32,
    inner_color: i32,
    outer_width: u32,
    outer_color: i32,
    elements: u32,
}

#[derive(Serialize, Deserialize)]
struct AreaYaml {

}

#[derive(Serialize, Deserialize)]
struct MapYaml {
    response_type: String,
    colors: ColorsYaml,
    symbols: SymbolsYaml,
}

pub fn map_to_yaml(map: &Map) -> Result<String, Box<dyn std::error::Error>>  {
    // COLORS
    let mut colors_yaml = ColorsYaml{
        num: map.colors.len(),
        colors: Vec::new()
    };
    for color in &map.colors.bag{
        colors_yaml.colors.push(
            ColorYaml{
                priority: color.priority,
                name: color.name.clone(),
                r:  color.r,
                g:  color.g,
                b:  color.b,
            }
        )
    }
    // SYMBOLS
    let mut symbols_yaml = SymbolsYaml{
        num:                map.symbols.len(),
        punctual_symbols:   Vec::new(),
        linear_symbols:     Vec::new(),
        area_symbols:       Vec::new(),
        text_symbols:       Vec::new(),
    };
    for symbol in &map.symbols.bag{
        if let Some(punctual_symbol) =(symbol as &dyn Any).downcast_ref::<PunctualSymbol>() {
            let mut annulus: Vec<AnnulusYaml> = Vec::new();
            let mut lines: Vec<LineYaml> = Vec::new();
            let mut areas: Vec<AreaYaml> = Vec::new();
            let mut texts: Vec<TextSymbolYaml> = Vec::new();
            for geometric_shape in &punctual_symbol.geometric_shapes{
                if let Some(an_annulus) =(&*geometric_shape as &dyn Any).downcast_ref::<Annulus>() {
                    let inner_color: i32;
                    match an_annulus.inner_color {
                        None => {inner_color = -1;}
                        Some(an_inner_color) => {inner_color = an_inner_color as i32;}
                    }
                    let outer_color: i32;
                    match an_annulus.outer_color {
                        None => {outer_color = -1;}
                        Some(an_outer_color) => {outer_color = an_outer_color as i32;}
                    }
                    annulus.push(
                        AnnulusYaml{
                            inner_radius: an_annulus.inner_radius,
                            inner_color,
                            outer_width: an_annulus.outer_width,
                            outer_color,
                            elements: an_annulus.elements,
                        }
                    )
                }else if let Some(line) =(&*geometric_shape as &dyn Any).downcast_ref::<Line>() {

                }else if let Some(area) =(&*geometric_shape as &dyn Any).downcast_ref::<Area>() {

                }else if let Some(text) =(&*geometric_shape as &dyn Any).downcast_ref::<TextSymbol>() {

                }
            }
            symbols_yaml.punctual_symbols.push(
                PunctualSymbolYaml{
                    id:             punctual_symbol.id,
                    code:           punctual_symbol.code.clone(),
                    name:           punctual_symbol.name.clone(),
                    description:    punctual_symbol.description.clone(),
                    annulus,
                    lines,
                    areas,
                    texts,
                }
            )
        }
    }
    let a_map_yaml = MapYaml{
        response_type: "map".to_string(),
        colors: colors_yaml,
        symbols: symbols_yaml,
    };
    Ok(serde_json::to_string_pretty(&a_map_yaml).unwrap())
}