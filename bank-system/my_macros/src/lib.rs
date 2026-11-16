// my_macros/src/lib.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn say_hello(input: TokenStream) -> TokenStream {
    // Ожидаем строковый литерал
    let msg = parse_macro_input!(input as syn::LitStr);

    let expanded = quote! {
        println!("{}", #msg);
    };

    expanded.into()
}