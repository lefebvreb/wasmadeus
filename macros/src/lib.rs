use proc_macro::TokenStream;

#[proc_macro]
pub fn js(input: TokenStream) -> TokenStream {
    todo!()
}

/// Builds a view.
/// 
/// # Formal DSL definition
/// 
/// Here is the syntax of this macro, as defined in an [EBNF form](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form).
/// 
/// ```txt
/// <view> ::= "view!" ~ "{" ~ <element>* ~ "}"
/// 
/// <element> ::= <tag> ~ "{" ~ <attr>* ~ <element>* ~ "}" ~ ","
/// 
/// <tag> ::= RUST_IDENTIFER
/// 
/// <attr> ::= RUST_IDENTIFIER ~ ":" ~ <expr> ~ ","
/// 
/// <expr> ::= RUST_EXPRESSION
/// ```
#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    todo!()
}