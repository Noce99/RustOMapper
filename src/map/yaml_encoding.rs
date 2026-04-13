use std::any::Any;
use serde::{Deserialize, Serialize};
use crate::map::Map;
use std::collections::HashMap;
use std::rc::Rc;
use crate::map::elements::{Element, PointNode};
use crate::map::symbols::area::AreaSymbol;
use crate::map::symbols::geometric_shape::{Ring, Area, Line, Circle};
use crate::map::symbols::linear::LinearSymbol;
use crate::map::symbols::punctual::PunctualSymbol;
use crate::map::symbols::SymbolsBag;
use crate::map::symbols::text::TextSymbol;
use crate::map::symbols::geometric_shape::GeometricShape;

#[derive(Serialize, Deserialize)]
struct MapYaml {
    response_type: String,
    colors: ColorsYaml,
    symbols: SymbolsYaml,
    elements: ElementsYaml,
    geometric_shapes: GeometricShapesYaml,
}

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
    punctual_symbols: HashMap<i32, PunctualSymbolYaml>,
    linear_symbols: HashMap<i32, LinearSymbolYaml>,
    area_symbols: HashMap<i32, AreaSymbolYaml>,
    text_symbols: HashMap<i32, TextSymbolYaml>,
}

#[derive(Serialize, Deserialize)]
struct PunctualSymbolYaml {
    id: i32,
    code: String,
    name: String,
    description: String,
    rings: Vec<Ring>,
    circles: Vec<Circle>,
    lines: Vec<Line>,
    areas: Vec<Area>,
    texts: Vec<TextSymbolYaml>,
}

#[derive(Serialize, Deserialize)]
struct AreaSymbolYaml {
    id: i32,
    code: String,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct TextSymbolYaml {
    id: i32,
    code: String,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct LinearSymbolYaml {
    id: i32,
    code: String,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct ElementsYaml {
    num: usize,
    elements: Vec<ElementYaml>,
}

#[derive(Serialize, Deserialize)]
struct ElementYaml {
    pub element_family: String,
    pub element_type: u32,
    pub symbol: i32,
    pub coordinates: Vec<PointNode>,
    pub pattern_rotation: f32,
    pub pattern_rotation_x: i64,
    pub pattern_rotation_y: i64,
}

impl ElementYaml {
    fn from_element(element: &Element, symbols: &SymbolsBag) -> Option<Self>{
        let symbol_option = symbols.symbol_by_id(element.symbol);

        match symbol_option {
            Some(a_symbol) => {
                Some(ElementYaml{
                    element_family: a_symbol.get_symbol_type(),
                    element_type: element.element_type,
                    symbol: element.symbol,
                    coordinates: element.coordinates.clone(),
                    pattern_rotation: element.pattern_rotation,
                    pattern_rotation_x: element.pattern_rotation_y,
                    pattern_rotation_y: element.pattern_rotation_x,
                })
            },
            None => {
                None
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct GeometricShapeBox {
    ring: Option<Ring>,
    circle: Option<Circle>,
    line: Option<Line>,
    area: Option<Area>,
    text: Option<TextSymbolYaml>
}

impl GeometricShapeBox {
    fn new(geometric_shape: Rc<dyn GeometricShape>) -> GeometricShapeBox{
        let mut ring: Option<Ring> = None;
        let mut circle: Option<Circle> = None;
        let mut line: Option<Line> = None;
        let mut area: Option<Area> = None;
        let text: Option<TextSymbolYaml> = None;

        if let Some(a_ring) =(geometric_shape.as_ref() as &dyn Any).downcast_ref::<Ring>() {
            ring = Some(
                Ring {
                    x: a_ring.x,
                    y: a_ring.y,
                    inner_radius: a_ring.inner_radius,
                    outer_width: a_ring.outer_width,
                    color: a_ring.color,
                }
            )
        }else if let Some(a_circle) =(geometric_shape.as_ref() as &dyn Any).downcast_ref::<Circle>() {
            circle = Some(
                Circle {
                    x: a_circle.x,
                    y: a_circle.y,
                    radius: a_circle.radius,
                    color: a_circle.color,
                }
            )
        }else if let Some(a_line) =(geometric_shape.as_ref() as &dyn Any).downcast_ref::<Line>() {
            line = Some(
                Line {
                    color: a_line.color,
                    line_width: a_line.line_width,
                    nodes: a_line.nodes.clone(),
                }
            )
        }else if let Some(an_area) =(geometric_shape.as_ref() as &dyn Any).downcast_ref::<Area>() {
            area = Some(
                Area {
                    color: an_area.color,
                    nodes: an_area.nodes.clone(),
                }
            )
        }else if let Some(_text) =(geometric_shape.as_ref() as &dyn Any).downcast_ref::<TextSymbol>() {

        }
        GeometricShapeBox{
            ring:ring,
            circle:circle,
            line:line,
            area:area,
            text:text,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct GeometricShapesYaml {
    num: usize,
    g_shapes: Vec<GeometricShapeBox>,
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
        punctual_symbols:   HashMap::new(),
        linear_symbols:     HashMap::new(),
        area_symbols:       HashMap::new(),
        text_symbols:       HashMap::new(),
    };
    for (id, symbol) in &map.symbols.bag{
        if let Some(punctual_symbol) =(symbol.as_ref() as &dyn Any).downcast_ref::<PunctualSymbol>() {
            let mut rings: Vec<Ring> = Vec::new();
            let mut circles: Vec<Circle> = Vec::new();
            let mut lines: Vec<Line> = Vec::new();
            let areas: Vec<Area> = Vec::new();
            let texts: Vec<TextSymbolYaml> = Vec::new();
            for geometric_shape in &punctual_symbol.geometric_shapes{
                if let Some(a_ring) =(geometric_shape.as_ref() as &dyn Any).downcast_ref::<Ring>() {
                    rings.push(
                        Ring {
                            x: a_ring.x,
                            y: a_ring.y,
                            inner_radius: a_ring.inner_radius,
                            outer_width: a_ring.outer_width,
                            color: a_ring.color,
                        }
                    )
                }else if let Some(a_circle) =(geometric_shape.as_ref() as &dyn Any).downcast_ref::<Circle>() {
                    circles.push(
                        Circle {
                            x: a_circle.x,
                            y: a_circle.y,
                            radius: a_circle.radius,
                            color: a_circle.color,
                        }
                    )
                }else if let Some(line) =(geometric_shape.as_ref() as &dyn Any).downcast_ref::<Line>() {
                    lines.push(
                        Line {
                            color: line.color,
                            line_width: line.line_width,
                            nodes: line.nodes.clone(),
                        }
                    );
                }else if let Some(_area) =(geometric_shape.as_ref() as &dyn Any).downcast_ref::<Area>() {

                }else if let Some(_text) =(geometric_shape.as_ref() as &dyn Any).downcast_ref::<TextSymbol>() {

                }
            }
            symbols_yaml.punctual_symbols.insert(
                id.clone(),
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
            );
        } else if let Some(linear_symbol) =(symbol.as_ref() as &dyn Any).downcast_ref::<LinearSymbol>() {
            symbols_yaml.linear_symbols.insert(
                id.clone(),
                LinearSymbolYaml{
                    id:             linear_symbol.id,
                    code:           linear_symbol.code.clone(),
                    name:           linear_symbol.name.clone(),
                    description:    linear_symbol.description.clone(),
                }
            );
        } else if let Some(area_symbol) =(symbol.as_ref() as &dyn Any).downcast_ref::<AreaSymbol>() {
            symbols_yaml.area_symbols.insert(
                id.clone(),
                AreaSymbolYaml{
                    id:             area_symbol.id,
                    code:           area_symbol.code.clone(),
                    name:           area_symbol.name.clone(),
                    description:    area_symbol.description.clone(),
                }
            );
        } else if let Some(text_symbol) =(symbol.as_ref() as &dyn Any).downcast_ref::<TextSymbol>() {
            symbols_yaml.text_symbols.insert(
                id.clone(),
                TextSymbolYaml{
                    id:             text_symbol.id,
                    code:           text_symbol.code.clone(),
                    name:           text_symbol.name.clone(),
                    description:    text_symbol.description.clone(),
                }
            );
        }
    }
    // ELEMENTS
    let mut elements_yaml = ElementsYaml{
        num: 0,
        elements: Vec::new(),
    };
    for element in &map.elements.bag {
        match ElementYaml::from_element(element, &map.symbols){
            None => {}
            Some(an_element) => {
                elements_yaml.elements.push(an_element);
                elements_yaml.num +=1;
            }
        }
    }

    // GEOMETRIC SHAPES
    let mut geometric_shapes_yaml = GeometricShapesYaml{
        num: 0,
        g_shapes: Vec::new(),
    };
    for geometric_shapes in &map.geometric_shapes.bag {
        let a_geometric_shape_box = GeometricShapeBox::new(geometric_shapes.clone());
        geometric_shapes_yaml.num += 1;
        geometric_shapes_yaml.g_shapes.push(a_geometric_shape_box);
    }


    let a_map_yaml = MapYaml{
        response_type: "map".to_string(),
        colors: colors_yaml,
        symbols: symbols_yaml,
        elements: elements_yaml,
        geometric_shapes: geometric_shapes_yaml,
    };
    Ok(serde_json::to_string_pretty(&a_map_yaml).unwrap())
}