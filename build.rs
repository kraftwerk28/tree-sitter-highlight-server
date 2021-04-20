use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

fn build_language_parser(grammar_path: PathBuf) -> io::Result<()> {
    let parser_dir = grammar_path.join("src");
    let parser_file = parser_dir.join("parser.c");
    let scanner_file = parser_dir.join("scanner.c");

    let mut builder = cc::Build::new();
    builder
        .include(parser_dir)
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

    let paths: Vec<_> = PathBuf::from("parsers")
        .read_dir()
        .expect("`parsers/` directory exists")
        .filter_map(Result::ok)
        .map(|it| it.path())
        .collect();

    let mut externs_file =
        fs::OpenOptions::new().write(true).create(true).open(
            PathBuf::from(env::var("OUT_DIR").unwrap())
                .join("tree_sitter_fns.rs"),
        )?;

    let languages: Vec<String> = paths
        .iter()
        .filter_map(|p| {
            Some(
                p.file_name()?
                    .to_str()?
                    .strip_prefix("tree-sitter-")?
                    .to_string(),
            )
        })
        .collect();

    externs_file.write(
        b"type P = (&'static str, unsafe extern \"C\" fn() -> ::tree_sitter::Language);\n
extern \"C\" {\n",
    )?;

    for (path, lang) in paths.iter().zip(languages.clone()) {
        build_language_parser(path.clone())?;
        externs_file.write(
            format!(
                "    fn tree_sitter_{}() -> ::tree_sitter::Language;\n",
                lang,
            )
            .as_bytes(),
        )?;
    }

    externs_file.write(b"}\n\n")?;
    externs_file.write(b"const __LANG_LIST: &[P] = &[\n")?;
    for lang in languages {
        externs_file.write(
            format!("    (\"{0}\", tree_sitter_{0}),\n", lang).as_bytes(),
        )?;
    }
    externs_file.write(b"];\n")?;
    Ok(())
}
