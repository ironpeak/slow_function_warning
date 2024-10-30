extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::Item;

#[proc_macro_attribute]
pub fn debug_slow_function_warning(arg: TokenStream, input: TokenStream) -> TokenStream {
    if !cfg!(debug_assertions) {
        return input;
    }
    slow_function_warning(arg, input)
}

#[proc_macro_attribute]
pub fn release_slow_function_warning(arg: TokenStream, input: TokenStream) -> TokenStream {
    if cfg!(debug_assertions) {
        return input;
    }
    slow_function_warning(arg, input)
}

#[proc_macro_attribute]
pub fn slow_function_warning(_arg: TokenStream, input: TokenStream) -> TokenStream {
    let Item::Fn(function) = syn::parse(input.clone()).unwrap() else {
        panic!("slow_function_warning can only be used on functions");
    };
    println!(
        "Slow function warning: {:?}",
        function.sig.ident.to_string()
    );
    function.into_token_stream().into()
}
