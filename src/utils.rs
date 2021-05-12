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
    ($name: literal, $mod: ident) => {
        crates_io_language!($name, $mod, language)
    };
    ($name: literal, $mod: ident, $func: ident) => {
        (
            $name,
            Lazy::new(|| {
                log::info!("Initializing {} (crates.io) language", $name);
                LanguageConfig {
                    language: ($mod::$func)(),
                    highlight_query: $mod::HIGHLIGHT_QUERY.to_string(),
                    injections_query: String::new(),
                    locals_query: String::new(),
                }
            }),
        )
    }; // ($mod: ident, injections) => {
       //     LanguageConfig {
       //         language: $mod::language,
       //         highlight_query: $mod::HIGHLIGHT_QUERY.to_string(),
       //         injections_query: $mod::INJECTION_QUERY.to_string(),
       //         locals_query: String::new(),
       //     }
       // };
       // ($mod: ident, injections, locals) => {
       //     LanguageConfig {
       //         language: $mod::language,
       //         highlight_query: $mod::HIGHLIGHT_QUERY.to_string(),
       //         injections_query: $mod::INJECTION_QUERY.to_string(),
       //         locals_query: $mod::LOCALS_QUERY.to_string(),
       //     }
       // };
}

macro_rules! submodule_language {
    ($name: literal, $func: ident) => {
        (
            $name,
            Lazy::new(|| {
                log::info!("Initializing {} (submodule) language", $name);
                let highlights_path = concat!(
                    "parsers/tree-sitter-",
                    $name,
                    "/queries/highlights.scm"
                );
                let injections_path = concat!(
                    "parsers/tree-sitter-",
                    $name,
                    "/queries/injections.scm"
                );
                let locals_path = concat!(
                    "parsers/tree-sitter-",
                    $name,
                    "/queries/locals.scm"
                );
                let cfg = LanguageConfig {
                    language: unsafe { $func() },
                    highlight_query: fs::read_to_string(highlights_path)
                        .expect("Highlights don't exist for that language"),
                    injections_query: fs::read_to_string(injections_path)
                        .unwrap_or(String::new()),
                    locals_query: fs::read_to_string(locals_path)
                        .unwrap_or(String::new()),
                };
                cfg
            }),
        );
    };
}

pub static USVG_TREE_OPTIONS: Lazy<usvg::Options> = Lazy::new(|| {
    let mut tree_opts = usvg::Options::default();
    log::info!("Initializing usvg options...");
    tree_opts.image_rendering = usvg::ImageRendering::OptimizeSpeed;
    tree_opts.shape_rendering = usvg::ShapeRendering::OptimizeSpeed;
    tree_opts
        .fontdb
        .load_font_file("assets/fonts/JetBrainsMono-Regular.ttf")
        .unwrap();
    tree_opts.fontdb.set_monospace_family("JetBrains Mono");
    tree_opts
});

extern "C" {
    fn tree_sitter_c() -> Language;
    fn tree_sitter_haskell() -> Language;
}

static LANGUAGE_LIST: [(&'static str, Lazy<LanguageConfig>); 7] = [
    crates_io_language!("javascript", tree_sitter_javascript),
    submodule_language!("c", tree_sitter_c),
    crates_io_language!("cpp", tree_sitter_cpp),
    crates_io_language!("rust", tree_sitter_rust),
    crates_io_language!("python", tree_sitter_python),
    submodule_language!("haskell", tree_sitter_haskell),
    crates_io_language!(
        "typescript",
        tree_sitter_typescript,
        language_typescript
    ),
];

pub fn get_language(name: &str) -> Option<&Lazy<LanguageConfig>> {
    LANGUAGE_LIST.iter().find_map(|(lang_name, cfg)| {
        if lang_name == &name {
            Some(cfg.clone())
        } else {
            None
        }
    })
}
