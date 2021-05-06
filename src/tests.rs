use once_cell::sync::Lazy;
use std::{fs, path::Path};
use tiny_skia::Pixmap;
use tree_sitter::Parser;
use tree_sitter_highlight::{Highlight, HighlightConfiguration, Highlighter};
use usvg::{FitTo, Options, Tree};

use crate::utils::get_language;
use crate::{
    stylesheet::Stylesheet, sublime_colors::SublimeColorScheme,
    svg_renderer::SvgRenderer,
};

#[test]
fn parser() {
    let config = get_language("rust").expect("Language parser exists");
    let mut parser = Parser::new();
    parser.set_language(config.language).expect("Set language");
    let source_code = r#"
function add(a, b) {
    return a + b;
}
    "#
    .trim();
    let tree = parser.parse(source_code, None).unwrap();
    println!("{}", tree.root_node().to_sexp());
}

const NAMES_FROM_README: [&str; 18] = [
    "attribute",
    "constant",
    "function.builtin",
    "function",
    "keyword",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "string",
    "string.special",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
];

#[test]
fn highlight() {
    let source_code = fs::read_to_string("sample.js")
        .unwrap()
        .replace('\t', "    ");
    let language_name = "javascript";

    let mut hl_cfg = {
        let cfg = get_language(language_name).unwrap();
        HighlightConfiguration::new(
            cfg.language,
            &cfg.highlight_query,
            &cfg.injections_query,
            &cfg.locals_query,
        )
        .unwrap()
    };
    let mut highlighter = Highlighter::new();

    // let hl_names = NAMES_FROM_README
    //     .iter()
    //     .map(|s| s.to_string())
    //     .collect::<Vec<_>>();
    let hl_names = hl_cfg.names().to_vec();

    println!("Builtin highlight queries:\n{}", hl_cfg.names().join("\n"));
    println!("Current highlight names:\n{}", hl_names.join("\n"));

    let svg_attributes: Vec<_> = hl_names
        .iter()
        .map(|name| format!(r#"class="{}""#, name.replace(".", " ")))
        .collect();

    hl_cfg.configure(&hl_names.to_vec());

    let events = highlighter
        .highlight(&hl_cfg, source_code.as_bytes(), None, |_| None)
        .unwrap();

    let attribute_callback = |hl: &Highlight| svg_attributes[hl.0].clone();
    let mut svg_renderer =
        SvgRenderer::new(source_code.clone(), &attribute_callback);

    let stylesheet = fs::read_to_string("ayu-vim.css").unwrap();
    svg_renderer.render(events, stylesheet).unwrap();

    let svg_path = Path::new("sample.svg");
    let png_path = svg_path.file_stem().unwrap().to_string_lossy() + ".png";

    fs::write(svg_path, svg_renderer.get_svg()).unwrap();

    let tree =
        Tree::from_data(&svg_renderer.get_svg().as_bytes(), &USVG_TREE_OPTIONS)
            .unwrap();
    let (width, height) = svg_renderer.get_picture_size();
    let mut pixmap = Pixmap::new(width as u32, height as u32).unwrap();
    println!("Rendering");
    resvg::render(&tree, FitTo::Original, pixmap.as_mut()).unwrap();
    println!("Saving");
    pixmap.save_png(png_path.as_ref()).unwrap();

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
    let raw = fs::read_to_string("ayu-dark.sublime-color-scheme").unwrap();
    let cl_scheme = SublimeColorScheme::parse(&raw).unwrap();
    println!("{}", &cl_scheme.build_stylesheet());
}

const USVG_TREE_OPTIONS: Lazy<usvg::Options> = Lazy::new(|| {
    let mut tree_opts = Options::default();
    tree_opts
        .fontdb
        .load_font_file("assets/JetBrainsMono-Regular.ttf")
        .unwrap();
    // tree_opts.fontdb.load_system_fonts();
    tree_opts.fontdb.set_monospace_family("JetBrains Mono");
    tree_opts
});

#[test]
fn resvg() {
    // height / width = 3/5
    // Width of one character = 48
    // Therefore 80 columns = 960 px
    let file_name = Path::new("sample.svg");
    let result_name = file_name.file_stem().unwrap().to_string_lossy() + ".png";
    println!("{:?} {:?}", file_name, result_name);
    let svg_content = fs::read(file_name).unwrap();

    println!("Building tree");
    let tree = Tree::from_data(&svg_content, &USVG_TREE_OPTIONS).unwrap();
    println!("Creating pixmap");
    let mut pixmap = Pixmap::new(960, 512).unwrap();
    println!("Rendering");
    resvg::render(&tree, FitTo::Original, pixmap.as_mut()).unwrap();
    println!("Saving png");
    pixmap.save_png(result_name.as_ref()).unwrap();
}
