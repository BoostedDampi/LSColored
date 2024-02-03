use colored::*;
use std::collections::HashMap;
use std::error::Error;

//a color can be a rgb or hex value
pub enum Color {
    RGB { r: u8, g: u8, b: u8 },
    Hex(String),
}

impl Color {
    //i need rgb values for the colored create, so i convert them
    fn convert(&self) -> Option<(u8, u8, u8)> {
        match self {
            Color::RGB { r, g, b } => Some((*r, *g, *b)),

            Color::Hex(hex) => {
                if hex.len() == 6 {
                    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                    Some((r, g, b))
                } else {
                    None
                }
            }
        }
    }
}

//this is the general colorscheme that will be used in the program
pub struct ColorScheme {
    pub colors: HashMap<String, Color>,
}
impl ColorScheme {
    pub fn new() -> ColorScheme {
        ColorScheme {
            colors: HashMap::new(),
        }
    }

    pub fn add_rgb(&mut self, elem_name: String, r: u8, g: u8, b: u8) {
        self.colors.insert(elem_name, Color::RGB { r, g, b });
    }

    pub fn parse_text(&self, elem_name: String, text: &str) -> Result<String, Box<dyn Error>> {
        let (r, g, b) = self.colors.get(&elem_name).unwrap().convert().unwrap();
        Ok(format!("{}", text.truecolor(r, g, b)))
    }

    //TODO import from TOML file, problem is i do not know how slow permanent file reads are
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::new()
    }
}
