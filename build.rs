use std::path::PathBuf;

const LANGUAGES: &[&str] = &["javascript", "rust"];

fn build_language_parser(language: &str) {
    let pkg = format!("tree-sitter-{}", language);
    let parser_dir = format!("./parsers/{}/src/", pkg);
    let parser_file = PathBuf::from(format!("{}/parser.c", parser_dir));
    let scanner_file = PathBuf::from(format!("{}/scanner.c", parser_dir));

    let mut builder = cc::Build::new();
    builder
        .include(parser_dir)
        .file(parser_file.canonicalize().unwrap());
    if scanner_file.exists() {
        builder.file(scanner_file.canonicalize().unwrap());
    }
    builder.compile(&pkg)
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    for lang in LANGUAGES.iter() {
        build_language_parser(lang);
    }
}
