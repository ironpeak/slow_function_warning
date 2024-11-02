extern crate proc_macro;

use std::time::Duration;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Colon, Semi},
    Block, Expr, FnArg, Ident, Item, ItemFn, Lit, LitStr, Pat, PatIdent, PatType, Result, Stmt,
    Token,
};

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
                suffix => {
                    return Err(syn::Error::new(
                        expr.span(),
                        format!("Unexpected a numeric literal suffix {}", suffix),
                    ))
                }
            },
            _ => {
                return Err(syn::Error::new(expr.span(), "Expected a numeric literal"));
            }
        },
        _ => {
            return Err(syn::Error::new(expr.span(), "Expected a numeric literal"));
        }
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
    let args: Punctuated<Expr, Token![,]> =
        parse_macro_input!(args with Punctuated::<Expr, Token![,]>::parse_separated_nonempty)
            .into_iter()
            .collect();
    let Item::Fn(function) = syn::parse(input).unwrap() else {
        panic!("slow_function_warning can only be used on functions");
    };
    let time = if let Some(time_expr) = args.get(0) {
        parse_time(time_expr).unwrap()
    } else {
        panic!("slow_function_warning missing time argument");
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

#[proc_macro_attribute]
#[cfg(feature = "bevy")]
pub fn debug_slow_system_warning(args: TokenStream, input: TokenStream) -> TokenStream {
    if !cfg!(debug_assertions) {
        return input;
    }
    slow_system_warning(args, input)
}

#[proc_macro_attribute]
#[cfg(feature = "bevy")]
pub fn release_slow_system_warning(args: TokenStream, input: TokenStream) -> TokenStream {
    if cfg!(debug_assertions) {
        return input;
    }
    slow_system_warning(args, input)
}

#[proc_macro_attribute]
#[cfg(feature = "bevy")]
pub fn slow_system_warning(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: Punctuated<Expr, Token![,]> =
        parse_macro_input!(args with Punctuated::<Expr, Token![,]>::parse_separated_nonempty)
            .into_iter()
            .collect();
    let Item::Fn(function) = syn::parse(input).unwrap() else {
        panic!("slow_system_warning can only be used on functions");
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
                warn!("{function}: ran for {millis}ms");
            }
            .into(),
        )
        .unwrap()
    };
    slow_function_warning_common(time, stmt, function)
}

fn slow_function_warning_common(time: Duration, stmt: Stmt, mut function: ItemFn) -> TokenStream {
    let nano_seconds = time.as_nanos();
    let function_name_ident = function.sig.ident.clone();
    let function_name = Lit::Str(LitStr::new(
        &function_name_ident.to_string(),
        Span::call_site(),
    ));
    let function_params: Vec<_> = function
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Receiver(_) => Ident::new("self", Span::call_site()),
            syn::FnArg::Typed(typed) => match &*typed.pat {
                syn::Pat::Ident(ident) => ident.ident.clone(),
                _ => unreachable!(),
            },
        })
        .collect();

    let mut result = ItemFn {
        attrs: function.attrs.clone(),
        vis: function.vis.clone(),
        sig: function.sig.clone(),
        block: Box::new(Block {
            brace_token: function.block.brace_token.clone(),
            stmts: vec![],
        }),
    };

    rename_function_self(&mut function, "_self_arg_renamed");

    result.block.stmts = vec![
        Stmt::Item(Item::Fn(function)),
        syn::parse(
            quote! {
                let start = std::time::Instant::now();
            }
            .into(),
        )
        .unwrap(),
        syn::parse(
            quote! {
                let result = #function_name_ident(#(#function_params),*);
            }
            .into(),
        )
        .unwrap(),
        syn::parse(
            quote! {
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
                    #stmt;
                }
            }
            .into(),
        )
        .unwrap(),
        syn::parse(
            quote! {
                return result;
            }
            .into(),
        )
        .unwrap(),
    ];

    result.into_token_stream().into()
}

fn rename_function_self(function: &mut ItemFn, ident: &str) {
    let Some(FnArg::Receiver(receiver)) = function.sig.inputs.iter().next() else {
        return;
    };
    println!("{}: {:?}", ident, receiver);
    function.sig.inputs[0] = FnArg::Typed(PatType {
        attrs: receiver.attrs.clone(),
        pat: Box::new(Pat::Ident(PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: Ident::new(ident, Span::call_site()),
            subpat: None,
        })),
        colon_token: Colon {
            spans: [Span::call_site()],
        },
        ty: receiver.ty.clone(),
    });
}
