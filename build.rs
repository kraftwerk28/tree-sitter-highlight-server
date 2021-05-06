use std::{io, path::PathBuf};

fn build_language_parser(grammar_path: PathBuf) -> io::Result<()> {
    let src_dir = grammar_path.join("src");
    let parser_file = src_dir.join("parser.c");
    let mut scanner_file = src_dir.join("scanner.c");
    if !scanner_file.exists() {
        scanner_file = src_dir.join("scanner.cc");
    }

    let mut builder = cc::Build::new();
    builder
        .include(src_dir)
        .flag("-Wno-unused")
        .file(parser_file.canonicalize()?);

    if scanner_file.exists() {
        builder.file(scanner_file.canonicalize()?);
    }
    builder.compile(grammar_path.file_stem().unwrap().to_str().unwrap());
    Ok(())
}

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    build_language_parser(PathBuf::from("parsers/tree-sitter-c"))?;
    build_language_parser(PathBuf::from("parsers/tree-sitter-haskell"))?;
    Ok(())
}
