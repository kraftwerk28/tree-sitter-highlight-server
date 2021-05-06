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

extern "C" {
    // extern functions for parsers/ directory
    fn tree_sitter_javascript() -> Language;
    // fn tree_sitter_cpp() -> Language;
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

// include!(concat!(env!("OUT_DIR"), "/tree_sitter_fns.rs"));

pub fn get_language(name: &str) -> Option<Lazy<LanguageConfig>> {
    match name {
        // "javascript" => Some(crates_io_language!(tree_sitter_javascript)),
        "javascript" => Some(submodule_language!("javascript", tree_sitter_javascript)),
        // "c" => Some(crates_io_language!(tree_sitter_c)),
        "cpp" => Some(crates_io_language!(tree_sitter_cpp)),
        // "cpp" => Some(submodule_language!("cpp", tree_sitter_cpp)),
        // "typescript" => {
        //     Some(submodule_language!("typescript", tree_sitter_typescript))
        // }
        "rust" => Some(crates_io_language!(tree_sitter_rust)),
        "python" => Some(crates_io_language!(tree_sitter_python)),
        _ => None,
    }
}
