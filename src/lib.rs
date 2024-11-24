extern crate proc_macro;

use std::time::Duration;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Semi, *};

fn parse_time(expr: &Expr) -> Result<Duration> {
    match expr {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            syn::Lit::Int(literal) => match literal.suffix() {
                "ns" => Ok(Duration::from_nanos(literal.base10_parse::<u64>()?)),
                "ms" => Ok(Duration::from_millis(literal.base10_parse::<u64>()?)),
                "s" => Ok(Duration::from_secs(literal.base10_parse::<u64>()?)),
                "m" => Ok(Duration::from_secs(literal.base10_parse::<u64>()? * 60)),
                "h" => Ok(Duration::from_secs(
                    literal.base10_parse::<u64>()? * 60 * 60,
                )),
                "d" => Ok(Duration::from_secs(
                    literal.base10_parse::<u64>()? * 60 * 60 * 24,
                )),
                "" => Ok(Duration::from_millis(literal.base10_parse::<u64>()?)),
                suffix => Err(syn::Error::new(
                    expr.span(),
                    format!("Unexpected a numeric literal suffix {}", suffix),
                )),
            },
            _ => Err(syn::Error::new(expr.span(), "Expected a numeric literal")),
        },
        _ => Err(syn::Error::new(expr.span(), "Expected a numeric literal")),
    }
}

#[proc_macro_attribute]
pub fn debug_slow_function_warning(args: TokenStream, input: TokenStream) -> TokenStream {
    if !cfg!(debug_assertions) {
        return input;
    }
    slow_function_warning(args, input)
}

#[proc_macro_attribute]
pub fn release_slow_function_warning(args: TokenStream, input: TokenStream) -> TokenStream {
    if cfg!(debug_assertions) {
        return input;
    }
    slow_function_warning(args, input)
}

#[proc_macro_attribute]
pub fn slow_function_warning(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: Punctuated<Expr, Token![,]> = if !args.is_empty() {
        parse_macro_input!(args with Punctuated::<Expr, Token![,]>::parse_separated_nonempty)
            .into_iter()
            .collect()
    } else {
        Punctuated::default()
    };
    let Item::Fn(function) = syn::parse(input).unwrap() else {
        panic!("slow_function_warning can only be used on functions");
    };
    let time = if let Some(time_expr) = args.get(0) {
        parse_time(time_expr).unwrap()
    } else {
        Duration::from_millis(1)
    };
    let stmt = if let Some(stmt) = args.get(1) {
        Stmt::Expr(stmt.clone(), Some(Semi::default()))
    } else {
        syn::parse(
            quote! {
                println!("Warning: {module}::{function}: ran for {millis}ms");
            }
            .into(),
        )
        .unwrap()
    };
    slow_function_warning_common(time, stmt, function)
}

fn slow_function_warning_common(time: Duration, stmt: Stmt, function: ItemFn) -> TokenStream {
    let nano_seconds = time.as_nanos();
    let function_name_ident = function.sig.ident.clone();
    let function_name = Lit::Str(LitStr::new(
        &function_name_ident.to_string(),
        Span::call_site(),
    ));

    let mut result = ItemFn {
        attrs: function.attrs.clone(),
        vis: function.vis.clone(),
        sig: function.sig.clone(),
        block: Box::new(Block {
            brace_token: function.block.brace_token,
            stmts: vec![],
        }),
    };

    let is_async = function.sig.asyncness.is_some();
    let block = function.block;

    let closure_decleration: Stmt = if is_async {
        syn::parse(
            quote! {
                let mut closure = move || async move {
                    #block
                };
            }
            .into(),
        )
        .unwrap()
    } else {
        syn::parse(
            quote! {
                let mut closure = move || {
                    #block
                };
            }
            .into(),
        )
        .unwrap()
    };

    let closure_call: Stmt = if is_async {
        syn::parse(
            quote! {
                let result = closure().await;
            }
            .into(),
        )
        .unwrap()
    } else {
        syn::parse(
            quote! {
                let result = closure();
            }
            .into(),
        )
        .unwrap()
    };

    result.block = syn::parse(
        quote! {{
            #closure_decleration
            let start = instant::Instant::now();
            #closure_call
            if start.elapsed().as_nanos() > #nano_seconds {
                let module = module_path!();
                let function = #function_name;
                let elapsed = start.elapsed();
                let ns = elapsed.as_nanos();
                let nanos = ns;
                let ms = elapsed.as_millis();
                let millis = ms;
                let s = elapsed.as_secs();
                let secs = s;
                #stmt
            }
            result
        }}
        .into(),
    )
    .unwrap();

    result.into_token_stream().into()
}
