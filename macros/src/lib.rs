use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    Ident, LitStr, Result,
};

struct LangList(Punctuated<Ident, Comma>);

impl Parse for LangList {
    fn parse(s: ParseStream) -> Result<Self> {
        Ok(Self(Punctuated::parse_terminated(s)?))
    }
}

#[proc_macro]
pub fn define_langs(item: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(item as LangList);
    let lang_name = parsed
        .0
        .iter()
        .map(|it| LitStr::new(&it.to_string(), it.span()));
    let func_name = parsed
        .0
        .iter()
        .map(|it| format_ident!("tree_sitter_{}", it));
    let func_name2 = func_name.clone();

    let expanded = quote! {
        extern "C" {
            #(fn #func_name() -> ::tree_sitter::Language;)*
        }
        const __LANG_LIST: &[
            (&'static str, unsafe extern "C" fn() -> ::tree_sitter::Language)
        ] = &[
            #((#lang_name, #func_name2)),*
        ];
        fn get_language(language: &str) ->
            ::std::option::Option<::tree_sitter::Language>
        {
            for (name, f) in __LANG_LIST.iter() {
                if name == &language {
                    let lang = unsafe { f() };
                    return Some(lang);
                }
            }
            None
        }
    };
    TokenStream::from(expanded)
}
