use std::any::{type_name, Any};
use serde::{Deserialize, Serialize, Serializer};
use crate::map::colors::ColorsBag;
use crate::map::Map;
use std::any::TypeId;
use crate::map::symbols::area::AreaSymbol;
use crate::map::symbols::geometric_shape::{Ring, Area, Line, Circle};
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
    rings: Vec<RingYaml>,
    circles: Vec<CircleYaml>,
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
struct RingYaml {
    pub inner_radius: u32,
    pub outer_width: u32,
    pub color: u32,
}

#[derive(Serialize, Deserialize)]
struct CircleYaml {
    pub radius: u32,
    pub color: u32,
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
        if let Some(punctual_symbol) =(symbol.as_ref() as &dyn Any).downcast_ref::<PunctualSymbol>() {
            let mut rings: Vec<RingYaml> = Vec::new();
            let mut circles: Vec<CircleYaml> = Vec::new();
            let mut lines: Vec<LineYaml> = Vec::new();
            let mut areas: Vec<AreaYaml> = Vec::new();
            let mut texts: Vec<TextSymbolYaml> = Vec::new();
            for geometric_shape in &punctual_symbol.geometric_shapes{
                if let Some(a_ring) =(geometric_shape.as_ref() as &dyn Any).downcast_ref::<Ring>() {
                    rings.push(
                        RingYaml {
                            inner_radius: a_ring.inner_radius,
                            outer_width: a_ring.outer_width,
                            color: a_ring.color,
                        }
                    )
                }else if let Some(a_circle) =(geometric_shape.as_ref() as &dyn Any).downcast_ref::<Circle>() {
                    circles.push(
                        CircleYaml {
                            radius: a_circle.radius,
                            color: a_circle.color,
                        }
                    )
                }else if let Some(line) =(geometric_shape.as_ref() as &dyn Any).downcast_ref::<Line>() {

                }else if let Some(area) =(geometric_shape.as_ref() as &dyn Any).downcast_ref::<Area>() {

                }else if let Some(text) =(geometric_shape.as_ref() as &dyn Any).downcast_ref::<TextSymbol>() {

                }
            }
            symbols_yaml.punctual_symbols.push(
                PunctualSymbolYaml{
                    id:             punctual_symbol.id,
                    code:           punctual_symbol.code.clone(),
                    name:           punctual_symbol.name.clone(),
                    description:    punctual_symbol.description.clone(),
                    rings,
                    circles,
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