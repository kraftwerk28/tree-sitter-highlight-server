use serde::Deserialize;
use std::collections::HashMap;

use crate::stylesheet::Stylesheet;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ThemeItem {
    JustColor(i32),
    Advanced {
        color: Option<i32>,
        bold: Option<bool>,
        underline: Option<bool>,
        italic: Option<bool>,
    },
}

#[derive(Debug, Deserialize)]
struct ColorDef {
    colorId: i32,
    hexString: String,
    name: String,
}

#[derive(Deserialize)]
pub struct SimpleColors(HashMap<String, String>);

impl Stylesheet for SimpleColors {
    fn build_stylesheet(&self) -> String {
        let mut basic = "body{".to_string();
        if let Some(color) = self.0.get("bg") {
            basic += &format!("background-color:{};", color);
        }
        if let Some(color) = self.0.get("fg") {
            basic += &format!("color:{};", color);
        }
        basic.push('}');
        let tokens = self
            .0
            .iter()
            .map(|(class, color)| format!(".{}{{color:{}}}", class, color))
            .collect::<Vec<_>>()
            .join("\n");
        format!("{}\n{}", basic, tokens)
    }
}
