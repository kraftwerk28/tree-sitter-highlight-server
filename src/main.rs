use std::{fs, path::Path};
use tree_sitter_highlight::HighlightConfiguration;

use macros::define_langs;

mod custom_colors;
mod stylesheet;
mod sublime_colors;
mod svg_renderer;

define_langs! {}

fn main() {}

fn create_highlight_configuration(
    language: &str,
) -> Option<HighlightConfiguration> {
    let raw_dir = format!("parsers/tree-sitter-{}/queries", language);
    let queries_dir = Path::new(&raw_dir).canonicalize().ok()?;
    let highlights = queries_dir.join("highlights.scm");
    let injections = queries_dir.join("injections.scm");
    let locals = queries_dir.join("locals.scm");
    if !highlights.exists() {
        return None;
    }
    let cfg = HighlightConfiguration::new(
        get_language(language)?,
        &fs::read_to_string(highlights).ok()?,
        &fs::read_to_string(injections).unwrap_or(String::new()),
        &fs::read_to_string(locals).unwrap_or(String::new()),
    )
    .ok()?;

    Some(cfg)
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
    };
    use tiny_skia::Pixmap;
    use tree_sitter::Parser;
    use tree_sitter_highlight::{Highlight, Highlighter, HtmlRenderer};
    use usvg::{FitTo, Options, Tree};

    use crate::{
        create_highlight_configuration, get_language, svg_renderer::SvgRenderer,
    };

    #[test]
    fn parser() {
        let language = get_language("javascript").unwrap();
        let mut parser = Parser::new();
        parser.set_language(language).unwrap();
        let source_code = "function add(a, b) { return a + b; }";
        let tree = parser.parse(source_code, None).unwrap();
        println!("{}", tree.root_node().to_sexp());
    }

    #[test]
    fn highlighter() {
        // let source = fs::read_to_string("macros/src/lib.rs").unwrap();
        let source = r#"
abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz00
        "#
        .trim();
        // function add(a) {
        //     return b => a + b;
        // }
        // console.log(add(2)(42));
        // console.log(add(2)(42));console.log(add(2)(42));console.log(add(2)(42));console.log(add(2)(42));
        // class Foo {
        //     constructor() {
        //         const o = { bruh: 123 };
        //     }
        // }
        // let source = fs::read_to_string(
        //     "/home/kraftwerk28/projects/js/blcklstbot/ioredis-test.js",
        // )
        // .unwrap();
        let language = "javascript";
        let mut cfg = create_highlight_configuration(language).unwrap();
        let mut highlighter = Highlighter::new();
        // let mut html_renderer = HtmlRenderer::new();
        let mut svg_renderer = SvgRenderer::new();

        let names = cfg.names().to_vec();
        println!("{}", names.join("\n"));

        let attrs: Vec<_> = names
            .iter()
            .map(|name| format!("class=\"{}\"", name.replace(".", " ")))
            .collect();
        cfg.configure(&names.to_vec());

        let events = highlighter
            .highlight(&cfg, source.as_bytes(), None, |_| None)
            .unwrap();

        // let attr_cb = |hl: Highlight| -> &[u8] { attrs[hl.0].as_bytes() };
        let attr_cb = |hl: Highlight| attrs[hl.0].clone();

        // html_renderer
        //     .render(events, source.as_bytes(), &attr_cb)
        //     .unwrap();

        let stylesheet = fs::read_to_string("atom_ayu_dark.css").unwrap();
        svg_renderer
            .render(&source, events, &attr_cb, stylesheet)
            .unwrap();

        println!(
            "GENERATED SVG:\n{}",
            svg_renderer.svg(),
            // String::from_utf8_lossy(&html_renderer.html)
        );
        fs::write(PathBuf::from("result.svg"), svg_renderer.svg()).unwrap();

        // let styles: SimpleColors =
        //     serde_json::from_str(&fs::read_to_string("ayu_dark.json").unwrap())
        //         .unwrap();

        // let prepared = format!(
        //     "<style>{}</style><pre><code>{}</code></pre>",
        //     styles,
        //     // styles.build_stylesheet(),
        //     String::from_utf8_lossy(&html_renderer.html)
        // );
        // let path_to_result = PathBuf::from("result.html");
        // fs::write(&path_to_result, prepared.as_bytes()).unwrap();
    }

    #[test]
    fn sublime_parsing() {
        use crate::stylesheet::Stylesheet;
        use crate::sublime_colors::SublimeColorScheme;
        use std::fs;

        let raw = fs::read_to_string("ayu-dark.sublime-color-scheme").unwrap();
        let cl_scheme = SublimeColorScheme::parse(&raw).unwrap();
        println!("{}", &cl_scheme.build_stylesheet());
    }

    #[test]
    fn resvg() {
        let content = fs::read("result.svg").unwrap();

        let mut tree_opts = Options::default();
        tree_opts.fontdb.load_system_fonts();
        tree_opts.fontdb.set_monospace_family("JetBrains Mono");

        let tree = Tree::from_data(&content, &tree_opts).unwrap();
        let mut pixmap = Pixmap::new(800, 200).unwrap();
        resvg::render(&tree, FitTo::Original, pixmap.as_mut()).unwrap();
        pixmap.save_png(Path::new("resvg_result.png")).unwrap();
    }
}
