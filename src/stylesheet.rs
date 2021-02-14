use std::fs;

use crate::sublime_colors::SublimeColorScheme;

pub trait Stylesheet {
    fn build_stylesheet(&self) -> String;
}

// TODO: make better theme
fn make_stylesheet() -> String {
    let raw = fs::read_to_string("ayu-mirage.sublime-color-scheme").unwrap();
    // let raw = fs::read_to_string("ayu-dark.sublime-color-scheme").unwrap();
    let cl_scheme = SublimeColorScheme::parse(&raw).unwrap();
    cl_scheme.build_stylesheet()
    // let default_theme_str = r#"
    //         {
    //           "attribute": {"color": 124, "italic": true},
    //           "comment": {"color": 245, "italic": true},
    //           "constant.builtin": {"color": 94, "bold": true},
    //           "constant": 94,
    //           "constructor": 136,
    //           "function.builtin": {"color": 26, "bold": true},
    //           "function.method": 33,
    //           "function": 26,
    //           "keyword": 56,
    //           "number": {"color": 94, "bold": true},
    //           "property": 124,
    //           "operator": {"color": 239, "bold": true},
    //           "punctuation.bracket": 239,
    //           "punctuation.delimiter": 239,
    //           "string.special": 30,
    //           "string": 28,
    //           "tag": 18,
    //           "type": 23,
    //           "type.builtin": {"color": 23, "bold": true},
    //           "variable.builtin": {"bold": true},
    //           "variable.parameter": {"underline": true}
    //         }
    //     "#;

    // let default_theme: HashMap<String, ThemeItem> =
    //     serde_json::from_str(default_theme_str).unwrap();

    // let color_map: HashMap<i32, String> =
    //     serde_json::from_str::<Vec<ColorDef>>(
    //         &fs::read_to_string("./term_colors.json").unwrap(),
    //     )
    //     .unwrap()
    //     .iter()
    //     .fold(HashMap::new(), |mut acc, item| {
    //         acc.insert(item.colorId, item.hexString.clone());
    //         acc
    //     });

    // let styles = default_theme.values().map(|v| match v {
    //     ThemeItem::JustColor(code) => {
    //         format!("color:{};", color_map[code])
    //     }
    //     ThemeItem::Advanced {
    //         color,
    //         bold,
    //         italic,
    //         underline,
    //     } => {
    //         let mut css = String::new();
    //         if let Some(code) = color {
    //             css.push_str(&format!("color:{};", color_map[code]));
    //         }
    //         if bold.unwrap_or(false) {
    //             css.push_str("font-weight:bold;");
    //         }
    //         if italic.unwrap_or(false) {
    //             css.push_str("font-style:italic;");
    //         }
    //         if underline.unwrap_or(false) {
    //             css.push_str("text-decoration:underline;");
    //         }
    //         css
    //     }
    // });
    // default_theme
    //     .keys()
    //     .zip(styles)
    //     .map(|(class, style)| format!(".{} {{{}}}", class, style))
    //     .collect::<Vec<_>>()
    //     .join("\n")
}
