use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::path::Path;
use syn::Ident;

#[proc_macro]
pub fn define_langs(_t: TokenStream) -> TokenStream {
    let entries: Vec<_> = Path::new("parsers/")
        .read_dir()
        .unwrap()
        .filter_map(|p| Some(p.ok()?.path()))
        .filter(|p| p.is_dir())
        .filter_map(|p| Some(p.file_name()?.to_str()?.to_string()))
        .collect();

    let func: Vec<_> = entries
        .iter()
        .map(|s| s.replace("-", "_"))
        .map(|s| Ident::new(&s, Span::call_site()))
        .collect();

    let lang = entries
        .iter()
        .map(|s| s.trim_start_matches("tree-sitter-").to_string());

    let expanded = quote! {
        extern "C" {
            #(fn #func() -> ::tree_sitter::Language;)*
        }

        const __LANG_LIST: &[
            (&'static str, unsafe extern "C" fn() -> ::tree_sitter::Language)
        ] = &[
            #((#lang, #func)),*
        ];

        pub fn get_language(language: &str) ->
            ::std::option::Option<::tree_sitter::Language>
        {
            __LANG_LIST.iter().find_map(|(name, func)| {
                if name == &language {
                    Some(unsafe { func() })
                } else {
                    None
                }
            })
        }
    };

    TokenStream::from(expanded)
}
