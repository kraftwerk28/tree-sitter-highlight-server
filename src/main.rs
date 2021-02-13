use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use tree_sitter::Language;

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

extern "C" {
    fn tree_sitter_javascript() -> Language;
    fn tree_sitter_rust() -> Language;
}

fn main() {}

fn configure_language(language_name: &str) {}

// TODO: make better theme
fn make_stylesheet() -> String {
    let default_theme_str = r#"
            {
              "attribute": {"color": 124, "italic": true},
              "comment": {"color": 245, "italic": true},
              "constant.builtin": {"color": 94, "bold": true},
              "constant": 94,
              "constructor": 136,
              "function.builtin": {"color": 26, "bold": true},
              "function.method": 33,
              "function": 26,
              "keyword": 56,
              "number": {"color": 94, "bold": true},
              "property": 124,
              "operator": {"color": 239, "bold": true},
              "punctuation.bracket": 239,
              "punctuation.delimiter": 239,
              "string.special": 30,
              "string": 28,
              "tag": 18,
              "type": 23,
              "type.builtin": {"color": 23, "bold": true},
              "variable.builtin": {"bold": true},
              "variable.parameter": {"underline": true}
            }
        "#;

    let default_theme: HashMap<String, ThemeItem> =
        serde_json::from_str(default_theme_str).unwrap();

    let color_map: HashMap<i32, String> =
        serde_json::from_str::<Vec<ColorDef>>(
            &fs::read_to_string("./term_colors.json").unwrap(),
        )
        .unwrap()
        .iter()
        .fold(HashMap::new(), |mut acc, item| {
            acc.insert(item.colorId, item.hexString.clone());
            acc
        });

    let styles = default_theme.values().map(|v| match v {
        ThemeItem::JustColor(code) => {
            format!("color:{};", color_map[code])
        }
        ThemeItem::Advanced {
            color,
            bold,
            italic,
            underline,
        } => {
            let mut css = String::new();
            if let Some(code) = color {
                css.push_str(&format!("color:{};", color_map[code]));
            }
            if bold.unwrap_or(false) {
                css.push_str("font-weight:bold;");
            }
            if italic.unwrap_or(false) {
                css.push_str("font-style:italic;");
            }
            if underline.unwrap_or(false) {
                css.push_str("text-decoration:underline;");
            }
            css
        }
    });
    default_theme
        .keys()
        .zip(styles)
        .map(|(class, style)| format!(".{} {{{}}}", class, style))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use crate::*;
    use tree_sitter::Parser;

    #[test]
    fn test_parser() {
        let language = unsafe { tree_sitter_javascript() };
        let mut parser = Parser::new();
        parser.set_language(language).unwrap();
        let source_code = "function add(a, b) { return a + b; }";
        let tree = parser.parse(source_code, None).unwrap();
        println!("{}", tree.root_node().to_sexp());
    }

    #[test]
    fn test_highlighter() {
        use std::fs::{self, read_to_string};
        use tree_sitter_highlight::{
            Highlight, HighlightConfiguration, Highlighter, HtmlRenderer,
        };

        // let language = unsafe { tree_sitter_javascript() };
        let language = unsafe { tree_sitter_rust() };

        // let source =
        //     "function add(a, b) {\n  return a + b;\n}\n\nconst foo = () => 42;\n\nclass Foo {}";
        let source = fs::read_to_string("src/main.rs").unwrap();

        // Javascript
        // let mut cfg = HighlightConfiguration::new(
        //     language,
        //     &read_to_string(
        //         "./parsers/tree-sitter-javascript/queries/highlights.scm",
        //     )
        //     .unwrap(),
        //     &read_to_string(
        //         "./parsers/tree-sitter-javascript/queries/injections.scm",
        //     )
        //     .unwrap(),
        //     &read_to_string(
        //         "./parsers/tree-sitter-javascript/queries/locals.scm",
        //     )
        //     .unwrap(),
        // )
        // .unwrap();

        let mut cfg = HighlightConfiguration::new(
            language,
            &read_to_string(
                "./parsers/tree-sitter-rust/queries/highlights.scm",
            )
            .unwrap(),
            &read_to_string(
                "./parsers/tree-sitter-rust/queries/injections.scm",
            )
            .unwrap(),
            "",
        )
        .unwrap();
        let mut highlighter = Highlighter::new();
        let mut html_renderer = HtmlRenderer::new();

        let names = cfg.names().to_vec();
        println!("{}", names.join("\n"));

        let attrs: Vec<_> = names
            .iter()
            .map(|name| format!("class=\"{}\"", name))
            .collect();
        cfg.configure(&names.to_vec());

        let events = highlighter
            .highlight(&cfg, source.as_bytes(), None, |_| None)
            .unwrap();

        let attr_cb = |hl: Highlight| -> &[u8] { attrs[hl.0].as_bytes() };

        html_renderer
            .render(events, source.as_bytes(), &attr_cb)
            .unwrap();

        println!(
            "GENERATED HTML:\n{}",
            String::from_utf8_lossy(&html_renderer.html)
        );

        let prepared = format!(
            "<style>{}</style><pre><code>{}</code></pre>",
            make_stylesheet(),
            String::from_utf8_lossy(&html_renderer.html)
        );
        fs::write("result.html", prepared.as_bytes()).unwrap();
    }
}
