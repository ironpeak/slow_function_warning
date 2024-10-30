extern crate proc_macro;

use std::time::Duration;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Semi,
    Expr, Item, Lit, LitStr, Result, Stmt, Token,
};

struct Args {
    time: Duration,
    stmt: Stmt,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut punctuated: Vec<Expr> =
            Punctuated::<Expr, Token![,]>::parse_separated_nonempty(input)?
                .into_iter()
                .collect();
        if punctuated.len() == 0 {
            return Err(syn::Error::new(input.span(), "Expected a numeric literal"));
        }
        let time = match &punctuated[0] {
            Expr::Lit(expr_lit) => match &expr_lit.lit {
                syn::Lit::Int(literal) => match literal.suffix() {
                    "ns" => Duration::from_nanos(literal.base10_parse::<u64>()?),
                    "ms" => Duration::from_millis(literal.base10_parse::<u64>()?),
                    "s" => Duration::from_secs(literal.base10_parse::<u64>()?),
                    "m" => Duration::from_secs(literal.base10_parse::<u64>()? * 60),
                    "h" => Duration::from_secs(literal.base10_parse::<u64>()? * 60 * 60),
                    "d" => Duration::from_secs(literal.base10_parse::<u64>()? * 60 * 60 * 24),
                    "" => Duration::from_millis(literal.base10_parse::<u64>()?),
                    suffix => {
                        return Err(syn::Error::new(
                            input.span(),
                            format!("Unexpected a numeric literal suffix {}", suffix),
                        ))
                    }
                },
                _ => {
                    return Err(syn::Error::new(input.span(), "Expected a numeric literal"));
                }
            },
            _ => {
                return Err(syn::Error::new(input.span(), "Expected a numeric literal"));
            }
        };
        let stmt = if punctuated.len() >= 2 {
            let expr: Expr = syn::parse(punctuated.pop().unwrap().to_token_stream().into())?;
            Stmt::Expr(expr, Some(Semi::default()))
        } else {
            syn::parse(
                quote! {
                println!("Warning: {module}::{function}: ran for {millis}ms"); }
                .into(),
            )?
        };
        Ok(Args { time, stmt })
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
    let args: Args = parse_macro_input!(args as Args);
    let Item::Fn(mut function) = syn::parse(input.clone()).unwrap() else {
        panic!("slow_function_warning can only be used on functions");
    };

    let stmt = args.stmt;
    let nano_seconds = args.time.as_nanos();
    let function_name = Lit::Str(LitStr::new(
        &function.sig.ident.to_string(),
        Span::call_site(),
    ));

    let mut stmts = vec![
        syn::parse(
            quote! {
                struct SlowFunctionWarning {
                    start: std::time::Instant,
                }
            }
            .into(),
        )
        .unwrap(),
        syn::parse(
            quote! {
                impl Drop for SlowFunctionWarning {
                    fn drop(&mut self) {
                        if self.start.elapsed().as_nanos() > #nano_seconds {
                            let module = module_path!();
                            let function = #function_name;
                            let elapsed = self.start.elapsed();
                            let ns = elapsed.as_nanos();
                            let nanos = ns;
                            let ms = elapsed.as_millis();
                            let millis = ms;
                            let s = elapsed.as_secs();
                            let secs = s;
                            #stmt;
                        }
                    }
                }
            }
            .into(),
        )
        .unwrap(),
        syn::parse(
            quote! {
                let _slow_function_warning = SlowFunctionWarning {
                    start: std::time::Instant::now(),
                };
            }
            .into(),
        )
        .unwrap(),
    ];
    function
        .block
        .stmts
        .drain(..)
        .for_each(|stmt| stmts.push(stmt));
    function.block.stmts = stmts;

    function.into_token_stream().into()
}
