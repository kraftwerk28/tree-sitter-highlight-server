use once_cell::sync::Lazy;
use std::fs;
use tree_sitter::Language;

#[derive(Debug)]
pub struct LanguageConfig {
    pub language: Language,
    pub highlight_query: String,
    pub injections_query: String,
    pub locals_query: String,
}

impl LanguageConfig {}

macro_rules! crates_io_language {
    ($mod: ident) => {
        Lazy::new(|| LanguageConfig {
            language: ($mod::language)(),
            highlight_query: $mod::HIGHLIGHT_QUERY.to_string(),
            injections_query: String::new(),
            locals_query: String::new(),
        })
    };
    ($mod: ident, injections) => {
        LanguageConfig {
            language: $mod::language,
            highlight_query: $mod::HIGHLIGHT_QUERY.to_string(),
            injections_query: $mod::INJECTION_QUERY.to_string(),
            locals_query: String::new(),
        }
    };
    ($mod: ident, injections, locals) => {
        LanguageConfig {
            language: $mod::language,
            highlight_query: $mod::HIGHLIGHT_QUERY.to_string(),
            injections_query: $mod::INJECTION_QUERY.to_string(),
            locals_query: $mod::LOCALS_QUERY.to_string(),
        }
    };
}

macro_rules! submodule_language {
    ($name: literal, $func: ident) => {
        Lazy::new(|| {
            let highlight_query = fs::read_to_string(concat!(
                "parsers/tree-sitter-",
                $name,
                "/queries/highlights.scm"
            ))
            .unwrap();
            let injections_query = fs::read_to_string(concat!(
                "parsers/tree-sitter-",
                $name,
                "/queries/injections.scm"
            ))
            .unwrap_or(String::new());
            let locals_query = fs::read_to_string(concat!(
                "parsers/tree-sitter-",
                $name,
                "/queries/locals.scm"
            ))
            .unwrap_or(String::new());
            let cfg = LanguageConfig {
                language: unsafe { $func() },
                highlight_query,
                injections_query,
                locals_query,
            };
            cfg
        });
    };
}

pub const USVG_TREE_OPTIONS: Lazy<usvg::Options> = Lazy::new(|| {
    let mut tree_opts = usvg::Options::default();
    tree_opts
        .fontdb
        .load_font_file("assets/fonts/JetBrainsMono-Regular.ttf")
        .unwrap();
    // tree_opts.fontdb.load_system_fonts();
    tree_opts.fontdb.set_monospace_family("JetBrains Mono");
    tree_opts
});

// include!(concat!(env!("OUT_DIR"), "/tree_sitter_fns.rs"));
extern "C" {
    // extern functions for parsers/ directory
    fn tree_sitter_c() -> Language;
    fn tree_sitter_haskell() -> Language;
}

pub fn get_language(name: &str) -> Option<Lazy<LanguageConfig>> {
    match name {
        "javascript" => Some(crates_io_language!(tree_sitter_javascript)),
        "c" => Some(submodule_language!("c", tree_sitter_c)),
        "cpp" => Some(crates_io_language!(tree_sitter_cpp)),
        "rust" => Some(crates_io_language!(tree_sitter_rust)),
        "python" => Some(crates_io_language!(tree_sitter_python)),
        "haskell" => Some(submodule_language!("haskell", tree_sitter_haskell)),
        _ => None,
    }
}
