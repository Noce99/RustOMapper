//! This file contains:
//! The definition and implementation of The Color and ColorsBag structs, and the parsing
//! of the `<colors>` section of a `.omap` file (including CMYK/RGB conversion and spot-color
//! tint composition) into them.

use crate::map_file::reading::Node;
use std::collections::HashMap;
use std::rc::Rc;

/// This struct describes a Color
pub struct Color {
    // The priority is used to identify which color should be on top of which others
    pub priority: u32,
    // The name of the Color
    pub name: String,
    // The r, g and b values in the range [0; 255]
    pub r:  u8,
    pub g:  u8,
    pub b:  u8,
    // The opacity/alpha of the color, in the range [0.; 1.]
    pub opacity: f32,
    // If it knockout all the other under color
    pub knockout: bool,
}

impl Color{
    /// Formats the color as a CSS/SVG-compatible `rgb(r, g, b)` string.
    pub fn get_string_rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

/// Converts a CMYK color (each channel in [0; 1]) to RGB (each channel in [0; 255]).
/// Uses the same naive conversion as Qt's `QColor::fromCmykF` (r = (1-c)(1-k), ...).
fn cmyk_to_rgb(c: f32, m: f32, y: f32, k: f32) -> (u8, u8, u8) {
    let r = (1. - c) * (1. - k);
    let g = (1. - m) * (1. - k);
    let b = (1. - y) * (1. - k);
    ((r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8)
}

/// This struct is used as the container (Bag) for all the colors defined in a .omap file
pub struct ColorsBag {
    // The colors that are used to create symbols
    pub colors: HashMap<u32, Color>,
}

impl ColorsBag {
    /// Parses the `<colors>` element of a `.omap` file into a `ColorsBag`.
    pub fn new(a_colors_node: Rc<Node>) -> Option<Self> {
        /*
        Example:
        color (priority:"34", name:"Yellow 50%", c:"0", m:"0.135", y:"0.395", k:"0", opacity:"1", ) {}
            spotcolors (knockout:"true", ) {}
                component (factor:"0.5", spotcolor:"32", ) {}
            cmyk (method:"spotcolor", ) {}
            rgb (method:"spotcolor", r:"1", g:"0.865", b:"0.605", ) {}

        cmyk and rgb are always there. spotcolors can also not be there.
        The final color shown on the map is always CMYK-based, unless cmyk's method is "rgb"
        (i.e. cmyk was derived from rgb), in which case the rgb value is used instead. Which
        CMYK value that is depends on cmyk's own method: the color's own c/m/y/k attributes
        when "custom", or a fresh composition from `spotcolors` when "spotcolor" (see below -
        the cached attributes are not the final value in that case).
        Knockout true means that when this (spot) color overlaps another one, it completely
        erases/replaces the color underneath instead of blending/darkening on top of it
        (simulated overprinting).
        Both cmyk and rgb have a `method` attribute, one of:
          - "custom": the value is given directly by its own attributes (c/m/y/k or r/g/b)
          - "spotcolor": the value is computed from the referenced `spotcolors` composition,
            in the color's own space (cmyk composes in CMYK, rgb composes in RGB - the two
            use different formulas and are not interchangeable)
          - for cmyk only, "rgb": the value is derived by converting the rgb value
          - for rgb only, "cmyk": the value is derived by converting the cmyk value
        rgb's r/g/b and cmyk's c/m/y/k attributes are cached values from the last time the map
        was saved: they hold the correct value for their respective "custom" method, but are
        stale/unused whenever the method is "spotcolor" (must be recomputed from the referenced
        spot colors) or when derived from the other color space.
        */
        if a_colors_node.name != "colors" {
            eprintln!("Parsing colors from a XML-Element that is not called colors?!");
            return None
        }
        let mut colors_bag: HashMap<u32, Color> = HashMap::new();
        // The cmyk of each base spot color, needed to compose tints in CMYK space (see
        // from_spotcolors_component_to_color): the original Mapper mixes spot-color tints by
        // accumulating CMYK channels, not by blending the already-converted RGB.
        let mut spotcolors_cmyk_bag: HashMap<u32, Cmyk> = HashMap::new();
        let mut spotcolor_defined_colors_bag: Vec<SpotcolorsDefinedColor> = Vec::new();
        // Colors whose rgb is a spotcolor-composed tint (cmyk method="rgb", rgb method="spotcolor"):
        // composed in RGB space, mirroring spotcolor_defined_colors_bag's CMYK-space composition.
        let mut rgb_spotcolor_defined_colors_bag: Vec<SpotcolorsDefinedColor> = Vec::new();
        for child in a_colors_node.children.borrow().iter() {
            let Some(attributes) = parse_color_attributes(child.clone()) else { continue };

            let cmyk_method = match child.search_child_by_name("cmyk") {
                Some(a_cmyk_node) => match parse_cmyk(a_cmyk_node){
                    Some(a_method) => a_method,
                    None => {eprintln!("Not possible to to parse an cmyk_node! {child}"); continue}
                },
                None => {eprintln!("Not possible to find a 'cmyk' child in the current Node! {child}"); continue}
            };

            let spotcolor_node = child.search_child_by_name("spotcolors");

            let color = match cmyk_method {
                CmykMethod::Custom => {
                    let a_cmyk = attributes.cmyk;
                    let (r, g, b) = cmyk_to_rgb(a_cmyk.c, a_cmyk.m, a_cmyk.y, a_cmyk.k);
                    Some(Color {
                        priority: attributes.priority,
                        name: attributes.name.clone(),
                        r,
                        g,
                        b,
                        opacity: attributes.opacity,
                        knockout: false,
                    })
                },
                CmykMethod::Rgb => {
                    let (rgb_method, rgb) = match child.search_child_by_name("rgb") {
                        Some(a_rgb_node) => match parse_rgb(a_rgb_node){
                            Some(a_rgb) => a_rgb,
                            None => {eprintln!("Not possible to to parse an rgb_node! {child}"); continue}
                        },
                        None => {eprintln!("Not possible to find a 'rgb' child in the current Node! {child}"); continue}
                    };
                    match rgb_method {
                        RgbMethod::SpotColor => {
                            if let Some(a_spotcolors_node) = &spotcolor_node {
                                let knockout = parse_knockout(a_spotcolors_node);
                                if let Some(components) = parse_spotcolor_definition(a_spotcolors_node){
                                    rgb_spotcolor_defined_colors_bag.push(
                                        SpotcolorsDefinedColor{
                                            components: components,
                                            priority: attributes.priority,
                                            name: attributes.name.clone(),
                                            opacity: attributes.opacity,
                                            knockout: knockout,
                                        }
                                    );
                                };
                            }
                            None
                        },
                        // RgbMethod::Cmyk here would mean cmyk is derived from rgb *and* rgb is
                        // derived from cmyk - a circular definition the format itself forbids.
                        // Fall back to the cached literal value rather than dropping the color.
                        RgbMethod::Custom | RgbMethod::Cmyk => Some(Color {
                            priority: attributes.priority,
                            name: attributes.name.clone(),
                            r: rgb.r,
                            g: rgb.g,
                            b: rgb.b,
                            opacity: attributes.opacity,
                            knockout: false,
                        })
                    }
                },
                CmykMethod::SpotColor => {
                    if let Some(a_spotcolors_node) = &spotcolor_node {
                        let knockout = parse_knockout(a_spotcolors_node);
                        if let Some(components) = parse_spotcolor_definition(a_spotcolors_node){
                            spotcolor_defined_colors_bag.push(
                                SpotcolorsDefinedColor{
                                    components: components,
                                    priority: attributes.priority,
                                    name: attributes.name.clone(),
                                    opacity: attributes.opacity,
                                    knockout: knockout,
                                }
                            );
                        };
                    }
                    None
                }
            };
            if let Some(a_spotcolors_node) = &spotcolor_node{
                if let Some(a_color) = &color{
                    if a_spotcolors_node.search_child_by_name("namedcolor").is_some(){
                        // This is a base spot ink (not a tint of one): register it for later
                        // tint composition instead of drawing it directly.
                        spotcolors_cmyk_bag.insert(attributes.priority, attributes.cmyk);
                        let a_color = Color {
                            priority: attributes.priority,
                            name: attributes.name.clone(),
                            r: a_color.r,
                            g: a_color.g,
                            b: a_color.b,
                            opacity: attributes.opacity,
                            knockout: parse_knockout(a_spotcolors_node),
                        };
                        colors_bag.insert(attributes.priority, a_color);
                        continue;
                    }
                }
            }
            if let Some(a_color) = color {
                colors_bag.insert(a_color.priority, a_color);
            }
        }


        for el in spotcolor_defined_colors_bag{
            let a_color = from_spotcolors_component_to_color(el, &spotcolors_cmyk_bag);
            colors_bag.insert(a_color.priority, a_color);
        }
        for el in rgb_spotcolor_defined_colors_bag{
            let a_color = from_rgb_spotcolors_component_to_color(el, &colors_bag);
            colors_bag.insert(a_color.priority, a_color);
        }

        Some(Self { colors: colors_bag})
    }

    /// Number of directly-defined colors (excludes the spotcolors ink table).
    pub fn len(&self) -> usize {
        self.colors.len()
    }
}


// Some helper functions
/// Parses a numeric XML attribute (e.g. `c`, `opacity`) as an `f32`.
fn parse_float_attribute(node: &Node, attribute: &str) -> Option<f32> {
    let raw = node.search_attribute_by_name(attribute)?;
    raw.parse().ok()
}
/// Parses a `[0;1]`-scaled float XML attribute (e.g. `r`, `g`, `b`) into a `[0;255]` channel value.
fn parse_u8_attribute(node: &Node, attribute: &str) -> Option<u8> {
    let raw = node.search_attribute_by_name(attribute)?;
    let value: f32 = raw.parse().ok()?;
    Some((value * 255.) as u8)
}
/// Reads the raw `method` attribute value off a `<cmyk>` or `<rgb>` node.
fn parse_method_str(node: &Node) -> Option<String> {
    match node.search_attribute_by_name("method") {
        Some(a_method) => Some(a_method),
        None => {eprintln!("Not able to find 'method' attribute in this Node! {node}"); None}
    }
}
/// Parses the `method` attribute of a `<cmyk>` node: one of "custom"/"spotcolor"/"rgb".
fn parse_cmyk_method(node: &Node) -> Option<CmykMethod>{
    let method_str = parse_method_str(node)?;
    match method_str.as_str() {
        "custom" => Some(CmykMethod::Custom),
        "spotcolor" => Some(CmykMethod::SpotColor),
        "rgb" => Some(CmykMethod::Rgb),
        _ => {eprintln!("A cmyk method is any of {{custom, spotcolor or rgb}} ! {method_str}"); None}
    }
}
/// Parses the `method` attribute of a `<rgb>` node: one of "custom"/"spotcolor"/"cmyk".
/// Note this is a different vocabulary from `parse_cmyk_method` - "cmyk" here plays the
/// role "rgb" plays there (derived from the other color space).
fn parse_rgb_method(node: &Node) -> Option<RgbMethod>{
    let method_str = parse_method_str(node)?;
    match method_str.as_str() {
        "custom" => Some(RgbMethod::Custom),
        "spotcolor" => Some(RgbMethod::SpotColor),
        "cmyk" => Some(RgbMethod::Cmyk),
        _ => {eprintln!("A rgb method is any of {{custom, spotcolor or cmyk}} ! {method_str}"); None}
    }
}
/// Parses the attributes carried directly by a `<color>` node itself: priority, name, its
/// cmyk value (meaningful only when the `<cmyk>` child's method is "custom") and opacity.
fn parse_color_attributes(color: Rc<Node>) -> Option<ColorAttributes>{
    let priority = match color.search_attribute_by_name("priority"){
        Some(a_priority) => match a_priority.parse(){
            Ok(b_priority) => b_priority,
            Err(_) => {eprintln!("Not able to parse 'priority' attribute in this Node! {color}"); return None}
        },
        None => {eprintln!("Not able to find 'priority' attribute in this Node! {color}"); return None}
    };
    let name = match color.search_attribute_by_name("name"){
        Some(a_name) => a_name,
        None => {eprintln!("Not able to find 'name' attribute in this Node! {color}"); return None}
    };

    let Some(c) = parse_float_attribute(&color, "c") else {
        eprintln!("Not able to find 'c' attribute in this Node! {color}"); return None
    };
    let Some(m) = parse_float_attribute(&color, "m") else {
        eprintln!("Not able to find 'm' attribute in this Node! {color}"); return None
    };
    let Some(y) = parse_float_attribute(&color, "y") else {
        eprintln!("Not able to find 'y' attribute in this Node! {color}"); return None
    };
    let Some(k) = parse_float_attribute(&color, "k") else {
        eprintln!("Not able to find 'k' attribute in this Node! {color}"); return None
    };
    let opacity = match parse_float_attribute(&color, "opacity"){
        Some(an_opacity) => an_opacity,
        None => 1.0,
    };

    Some(ColorAttributes {
        priority,
        name,
        cmyk: Cmyk { c, m, y, k },
        opacity,
    })
}
/// Parses a `<cmyk>` node. It only ever carries a `method` attribute - the actual c/m/y/k
/// values live on the parent `<color>` node (see `parse_color_attributes`).
fn parse_cmyk(cmyk: Rc<Node>) -> Option<CmykMethod> {
    parse_cmyk_method(&cmyk)
}
/// Reads the `knockout` attribute off a `<spotcolors>` node, defaulting to `false` when
/// missing or unparsable.
fn parse_knockout(spotcolors: &Rc<Node>) -> bool {
    match spotcolors.search_attribute_by_name("knockout") {
        Some(knockout_value) => knockout_value.parse::<bool>().unwrap_or(false),
        None => false,
    }
}
/// Parses a `<rgb>` node: its `method` plus its r/g/b attributes, which are always present
/// regardless of method (but only authoritative when the method is "custom").
fn parse_rgb(rgb: Rc<Node>) -> Option<(RgbMethod, Rgb)> {
    let Some(method) = parse_rgb_method(&rgb) else {
        return None
    };
    let Some(r) = parse_u8_attribute(&rgb, "r") else {
        eprintln!("Not able to find 'r' attribute in this Node! {rgb}"); return None
    };
    let Some(g) = parse_u8_attribute(&rgb, "g") else {
        eprintln!("Not able to find 'g' attribute in this Node! {rgb}"); return None
    };
    let Some(b) = parse_u8_attribute(&rgb, "b") else {
        eprintln!("Not able to find 'b' attribute in this Node! {rgb}"); return None
    };
    Some((method, Rgb{r, g, b}))
}
/// Parses the `<component>` children of a `<spotcolors>` node into the list of spot-color
/// mix components that make up a tint (returns `None` if there are none, e.g. for a base
/// named spot color, whose `<spotcolors>` node holds a `<namedcolor>` instead).
fn parse_spotcolor_definition(spotcolor: &Rc<Node>) -> Option<Vec<Component>> {
    let mut components: Vec<Component> = Vec::new();
    for child in spotcolor.children.borrow().iter(){
        if child.name != "component"{
            continue;
        }
        let factor = child.search_attribute_by_name("factor")
            .and_then(|a_value| a_value.parse::<f32>().ok());
        let spotcolor = child.search_attribute_by_name("spotcolor")
            .and_then(|a_value| a_value.parse::<u32>().ok());

        if let (Some(factor), Some(spotcolor)) = (factor, spotcolor) {
            components.push(Component { factor, spotcolor });
        }
    }
    if components.len() == 0{
        return None
    } else {
        return Some(components)
    }
}
/// Composes a tint/mix of spot colors the same way the original Mapper does: by accumulating
/// CMYK channels (not by blending the already-converted RGB), then converting the result to RGB.
fn from_spotcolors_component_to_color(scdc: SpotcolorsDefinedColor, spotcolors_cmyk: &HashMap<u32, Cmyk>) -> Color{
    let mut c: f32 = 0.;
    let mut m: f32 = 0.;
    let mut y: f32 = 0.;
    let mut k: f32 = 0.;
    for component in &scdc.components {
        let Some(other) = spotcolors_cmyk.get(&component.spotcolor) else {
            eprintln!("Spot color {} not found while composing color '{}' (priority {})!", component.spotcolor, scdc.name, scdc.priority);
            continue;
        };
        c += component.factor * other.c * (1. - c);
        m += component.factor * other.m * (1. - m);
        y += component.factor * other.y * (1. - y);
        k += component.factor * other.k * (1. - k);
    }
    let (r, g, b) = cmyk_to_rgb(c, m, y, k);
    Color {
        priority: scdc.priority,
        name: scdc.name,
        r,
        g,
        b,
        opacity: scdc.opacity,
        knockout: scdc.knockout,
    }
}

/// Composes a tint/mix of spot colors in RGB space (mirrors `MapColor::rgbFromSpotColors`),
/// for colors whose rgb method is "spotcolor" - the counterpart of
/// `from_spotcolors_component_to_color`, which composes in CMYK space instead.
fn from_rgb_spotcolors_component_to_color(scdc: SpotcolorsDefinedColor, spotcolors: &HashMap<u32, Color>) -> Color{
    let mut r: f32 = 1.;
    let mut g: f32 = 1.;
    let mut b: f32 = 1.;
    for component in &scdc.components {
        let Some(other) = spotcolors.get(&component.spotcolor) else {
            eprintln!("Spot color {} not found while composing color '{}' (priority {})!", component.spotcolor, scdc.name, scdc.priority);
            continue;
        };
        r *= 1. - component.factor * (1. - other.r as f32 / 255.);
        g *= 1. - component.factor * (1. - other.g as f32 / 255.);
        b *= 1. - component.factor * (1. - other.b as f32 / 255.);
    }
    Color {
        priority: scdc.priority,
        name: scdc.name,
        r: (r * 255.) as u8,
        g: (g * 255.) as u8,
        b: (b * 255.) as u8,
        opacity: scdc.opacity,
        knockout: scdc.knockout,
    }
}

// Other helper structs
/// A CMYK color, each channel in `[0; 1]`.
#[derive(Clone, Copy)]
struct Cmyk{
    c: f32,
    m: f32,
    y: f32,
    k: f32,
}

/// An RGB color as parsed from a `<rgb>` node, each channel in `[0; 255]`.
struct Rgb{
    r: u8,
    g: u8,
    b: u8,
}

/// Where a `<cmyk>` node's value actually comes from.
enum CmykMethod{
    /// The value is given directly by the color's own c/m/y/k attributes.
    Custom,
    /// The value is computed by composing the referenced `<spotcolors>` components in CMYK
    /// space (see `from_spotcolors_component_to_color`).
    SpotColor,
    /// The value is derived by converting the sibling `<rgb>` node's value.
    Rgb,
}

/// Where a `<rgb>` node's value actually comes from. Not the same vocabulary as
/// `CmykMethod`: an `<rgb>` node's method is one of "custom"/"spotcolor"/"cmyk" (never
/// "rgb"), so this can't share a single enum with the cmyk side.
enum RgbMethod{
    /// The value is given directly by the node's own r/g/b attributes.
    Custom,
    /// The value is computed by composing the referenced `<spotcolors>` components in RGB
    /// space (see `from_rgb_spotcolors_component_to_color`).
    SpotColor,
    /// The value is derived by converting the parent `<color>`'s c/m/y/k attributes.
    Cmyk,
}

/// The attributes carried directly by a `<color>` node (priority, name, cmyk, opacity).
struct ColorAttributes{
    priority: u32,
    name: String,
    cmyk: Cmyk,
    opacity: f32,
}

/// One `<component>` of a spot-color tint: a fraction (`factor`) of another color,
/// referenced by its `priority` (`spotcolor`).
struct Component{
    factor: f32,
    spotcolor: u32,
}

/// A color still awaiting composition from its spot-color tint `components`, deferred until
/// every base spot color has been parsed (see the two resolution loops in `ColorsBag::new`).
struct SpotcolorsDefinedColor{
    components: Vec<Component>,
    priority: u32,
    name: String,
    opacity: f32,
    knockout: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    // End-to-end regression check: none of the 46 <color> elements in the sample map should
    // be dropped while parsing. 9 of them are base named spot inks (<namedcolor>), which land
    // in `spotcolors` (they're composition references, not drawn directly) rather than in
    // `colors` - the remaining 37 are the actually-drawn colors.
    #[test]
    fn parses_all_colors_in_piccolo_paradiso() {
        let node = Node::node_from_file(concat!(env!("CARGO_MANIFEST_DIR"), "/../Maps/PiccoloParadiso.omap"))
            .expect("failed to read the sample map file");
        let map_node = node.search_child_by_name("map").expect("missing <map>");
        let colors_node = map_node.search_child_by_name("colors").expect("missing <colors>");
        let colors_bag = ColorsBag::new(colors_node).expect("failed to parse colors");

        assert_eq!(colors_bag.len(), 46);
    }

    // "Marrone 50%" from Maps/PiccoloParadiso.omap: factor 0.5 of spot color 35 "Marrone"
    // (cmyk 0, 0.56, 1, 0.18). The map's own cached rgb for this color is (232, 167, 116),
    // which this composition should reproduce.
    #[test]
    fn composes_spot_color_tint_in_cmyk_space() {
        let mut spotcolors_cmyk = HashMap::new();
        spotcolors_cmyk.insert(35, Cmyk { c: 0., m: 0.56, y: 1., k: 0.18 });

        let scdc = SpotcolorsDefinedColor {
            components: vec![Component { factor: 0.5, spotcolor: 35 }],
            priority: 22,
            name: "Marrone 50%".to_string(),
            opacity: 1.,
            knockout: false,
        };

        let color = from_spotcolors_component_to_color(scdc, &spotcolors_cmyk);

        assert_eq!((color.r, color.g, color.b), (232, 167, 116));
    }

    // Same base spot color as above, but resolved as an rgb method="spotcolor" tint instead
    // (composed in RGB space via the "screen" formula). This is a different, independently
    // correct formula from the CMYK-space one above - it's expected to diverge whenever the
    // base color has both a chromatic and a K component, as it does here.
    #[test]
    fn composes_rgb_spotcolor_tint_in_rgb_space() {
        let (r, g, b) = cmyk_to_rgb(0., 0.56, 1., 0.18);
        let mut spotcolors = HashMap::new();
        spotcolors.insert(35, Color {
            priority: 35,
            name: "Marrone".to_string(),
            r, g, b,
            opacity: 1.,
            knockout: false,
        });

        let scdc = SpotcolorsDefinedColor {
            components: vec![Component { factor: 0.5, spotcolor: 35 }],
            priority: 2,
            name: "Marrone 50% (rgb space)".to_string(),
            opacity: 1.,
            knockout: false,
        };

        let color = from_rgb_spotcolors_component_to_color(scdc, &spotcolors);

        assert_eq!((color.r, color.g, color.b), (232, 173, 127));
    }
}