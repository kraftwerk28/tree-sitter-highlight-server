use std::path::PathBuf;

fn build_language_parser(grammar_path: PathBuf) {
    let parser_dir = grammar_path.join("src");
    let parser_file = parser_dir.join("parser.c");
    let scanner_file = parser_dir.join("scanner.c");

    let mut builder = cc::Build::new();
    builder
        .include(parser_dir)
        .flag("-Wno-unused")
        .file(parser_file.canonicalize().unwrap());

    if scanner_file.exists() {
        builder.file(scanner_file.canonicalize().unwrap());
    }
    builder.compile(grammar_path.file_stem().unwrap().to_str().unwrap());
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    PathBuf::from("parsers")
        .read_dir()
        .expect("`parsers/` directory exists")
        .filter(Result::is_ok)
        .map(|it| it.unwrap().path())
        .for_each(build_language_parser);
}
