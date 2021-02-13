use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;

use crate::Stylesheet;

#[derive(Deserialize, Debug)]
pub struct SublimeColorScheme {
    name: String,
    globals: HashMap<String, String>,
    rules: Vec<Rule>,
}

#[derive(Deserialize, Debug)]
struct Rule {
    name: Option<String>,
    scope: Scopes,
    background: Option<Color>,
    foreground: Option<Color>,
    #[serde(default)]
    font_style: FontStyle,
}

#[derive(Deserialize, Debug)]
struct Color(String);

#[derive(Debug)]
struct Scopes(Vec<String>);

#[derive(Debug, Default)]
struct FontStyle {
    italic: bool,
    bold: bool,
    underline: bool,
}

impl<'de> de::Deserialize<'de> for Scopes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Vis;
        impl<'de> Visitor<'de> for Vis {
            type Value = Scopes;
            fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                unreachable!()
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Scopes(
                    v.split(&[',', ' '] as &[char])
                        .filter(|s| !s.is_empty())
                        .map(|part| part.trim().to_string())
                        .collect(),
                ))
            }
        }
        deserializer.deserialize_str(Vis)
    }
}

impl<'a> Deserialize<'a> for FontStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        struct Vis;
        impl<'b> Visitor<'b> for Vis {
            type Value = FontStyle;
            fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                unreachable!()
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let (mut italic, mut bold, mut underline) =
                    (false, false, false);
                if v.contains("bold") {
                    bold = true;
                }
                if v.contains("italic") {
                    italic = true;
                }
                if v.contains("underline") {
                    underline = true;
                }
                let font = FontStyle {
                    bold,
                    italic,
                    underline,
                };
                Ok(font)
            }
        }
        deserializer.deserialize_str(Vis)
    }
}

impl SublimeColorScheme {
    pub fn parse(raw: &str) -> serde_json::Result<Self> {
        serde_json::from_str(raw)
    }

    fn serialize_rule(rule: &Rule) -> String {
        let selector = rule
            .scope
            .0
            .iter()
            .map(|selector| format!(".{}", selector))
            // .map(|selector| {
            //     let sub_sel: Vec<_> = selector.split(".").collect();
            //     let len = sub_sel.len();
            //     let mut selectors = Vec::with_capacity(sub_sel.len());
            //     for i in 0..len {
            //         let partial =
            //             format!(".{}", &sub_sel[..len - i].to_vec().join("-"));
            //         selectors.push(partial);
            //     }
            //     selectors.join(",")
            // })
            .collect::<Vec<_>>()
            .join(",");
        let mut decl = '{'.to_string();
        if let Some(color) = &rule.background {
            decl += &format!("background-color:{};", color.0);
        }
        if let Some(color) = &rule.foreground {
            decl += &format!("color:{};", color.0);
        }
        if rule.font_style.bold {
            decl += "font-weight:bold;";
        }
        if rule.font_style.italic {
            decl += "font-weight:italic;";
        }
        if rule.font_style.underline {
            decl += "text-decoration:underline;";
        }
        decl.push('}');
        selector + &decl
    }
}

impl Stylesheet for SublimeColorScheme {
    fn build_stylesheet(&self) -> String {
        let mut basic = "body{".to_string();
        basic += "font-family:'JetBrains Mono', monospace;";
        if let Some(color) = self.globals.get("background") {
            basic += &format!("background:{};", color);
        }
        if let Some(color) = self.globals.get("foreground") {
            basic += &format!("color:{};", color);
        }
        basic.push('}');
        let token_rules = self
            .rules
            .iter()
            .map(SublimeColorScheme::serialize_rule)
            .collect::<Vec<_>>()
            .join("\n");
        format!("{}\n{}", basic, token_rules)
    }
}
