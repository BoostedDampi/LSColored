use colored::*;
use std::collections::HashMap;
use std::error::Error;

pub struct ColorScheme {

    pub colors: HashMap<String, CustomColor>,
}
impl ColorScheme {

    pub fn new() -> ColorScheme {
        ColorScheme {colors: HashMap::new()}
    }

    pub fn add_color(&mut self, elem_name: String, r:u8, g:u8, b:u8) {

        self.colors.insert(elem_name, colored::CustomColor {r,g,b});

    }

    pub fn parse_text(&self, elem_name: String, text: &str) -> Result<String, Box<dyn Error>> {
        Ok(format!("{}", text.custom_color(*self.colors.get(&elem_name).unwrap())))
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::new()
    }
}