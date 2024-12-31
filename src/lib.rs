extern crate proc_macro;

use std::time::Duration;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Semi, *};

enum TimeUnit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
}

impl TimeUnit {
    fn to_duration(&self, amount: u64) -> Duration {
        match self {
            TimeUnit::Nanoseconds => Duration::from_nanos(amount),
            TimeUnit::Microseconds => Duration::from_micros(amount),
            TimeUnit::Milliseconds => Duration::from_millis(amount),
            TimeUnit::Seconds => Duration::from_secs(amount),
            TimeUnit::Minutes => Duration::from_secs(amount * 60),
            TimeUnit::Hours => Duration::from_secs(amount * 60 * 60),
            TimeUnit::Days => Duration::from_secs(amount * 60 * 60 * 24),
        }
    }
}

fn parse_time(expr: &Expr) -> Result<(u64, TimeUnit)> {
    match expr {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            syn::Lit::Int(literal) => {
                let amount = literal.base10_parse::<u64>()?;
                let unit = match literal.suffix() {
                    "ns" => TimeUnit::Nanoseconds,
                    "us" | "μs" => TimeUnit::Microseconds,
                    "ms" => TimeUnit::Milliseconds,
                    "s" => TimeUnit::Seconds,
                    "m" => TimeUnit::Minutes,
                    "h" => TimeUnit::Hours,
                    "d" => TimeUnit::Days,
                    "" => TimeUnit::Milliseconds,
                    suffix => {
                        return Err(syn::Error::new(
                            expr.span(),
                            format!("Unexpected a numeric literal suffix {}", suffix),
                        ))
                    }
                };
                Ok((amount, unit))
            }
            _ => Err(syn::Error::new(expr.span(), "Expected a numeric literal")),
        },
        _ => Err(syn::Error::new(expr.span(), "Expected a numeric literal")),
    }
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

    let (time, unit) = if let Some(time_expr) = args.get(0) {
        parse_time(time_expr).unwrap()
    } else {
        (1, TimeUnit::Milliseconds)
    };

    let stmt = if let Some(stmt) = args.get(1) {
        Stmt::Expr(stmt.clone(), Some(Semi::default()))
    } else {
        syn::parse(
            quote! {
                println!("Warning: {module}::{function}: ran for {elapsed_str} (limit: {limit_str})");
            }
            .into(),
        )
        .unwrap()
    };

    let duration = unit.to_duration(time);
    let nano_seconds = duration.as_nanos();
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

    let elapsed_str = match unit {
        TimeUnit::Nanoseconds => quote! {
            format!("{}ns", elapsed.as_nanos())
        },
        TimeUnit::Microseconds => quote! {
            format!("{}μs", elapsed.as_micros())
        },
        TimeUnit::Milliseconds => quote! {
            format!("{}ms", elapsed.as_millis())
        },
        TimeUnit::Seconds => quote! {
            format!("{}s", elapsed.as_secs())
        },
        TimeUnit::Minutes => quote! {
            format!("{}m", elapsed.as_secs() / 60)
        },
        TimeUnit::Hours => quote! {
            format!("{}h", elapsed.as_secs() / 60 / 60)
        },
        TimeUnit::Days => quote! {
            format!("{}d", elapsed.as_secs() / 60 / 60 / 24)
        },
    };

    let limit_str = match unit {
        TimeUnit::Nanoseconds => quote! {
            format!("{}ns", limit.as_nanos())
        },
        TimeUnit::Microseconds => quote! {
            format!("{}μs", limit.as_micros())
        },
        TimeUnit::Milliseconds => quote! {
            format!("{}ms", limit.as_millis())
        },
        TimeUnit::Seconds => quote! {
            format!("{}s", limit.as_secs())
        },
        TimeUnit::Minutes => quote! {
            format!("{}m", limit.as_secs() / 60)
        },
        TimeUnit::Hours => quote! {
            format!("{}h", limit.as_secs() / 60 / 60)
        },
        TimeUnit::Days => quote! {
            format!("{}d", limit.as_secs() / 60 / 60 / 24)
        },
    };

    result.block = syn::parse(
        quote! {{
            #closure_decleration
            #[cfg(not(target_family = "wasm"))]
            let start = std::time::Instant::now();
            #[cfg(target_family = "wasm")]
            let start = web_time::Instant::now();
            #closure_call
            if start.elapsed().as_nanos() > #nano_seconds {
                let module = module_path!();
                let function = #function_name;

                let elapsed = start.elapsed();
                let elapsed_str = #elapsed_str;
                let elapsed_ns = elapsed.as_nanos();
                let elapsed_nanos = elapsed_ns;
                let elapsed_nanoseconds = elapsed_ns;
                let elapsed_us = elapsed.as_micros();
                let elapsed_micros = elapsed_us;
                let elapsed_microseconds = elapsed_us;
                let elapsed_ms = elapsed.as_millis();
                let elapsed_millis = elapsed_ms;
                let elapsed_milliseconds = elapsed_ms;
                let elapsed_s = elapsed.as_secs();
                let elapsed_secs = elapsed_s;
                let elapsed_seconds = elapsed_s;
                let elapsed_m = elapsed.as_secs() / 60;
                let elapsed_min = elapsed_m;
                let elapsed_minutes = elapsed_m;
                let elapsed_h = elapsed.as_secs() / 60 / 60;
                let elapsed_hours = elapsed_h;
                let elapsed_d = elapsed.as_secs() / 60 / 60 / 24;
                let elapsed_days = elapsed_d;

                let limit = std::time::Duration::from_nanos(#nano_seconds as u64);
                let limit_str = #limit_str;
                let limit_ns = limit.as_nanos();
                let limit_nanos = limit_ns;
                let limit_nanoseconds = limit_ns;
                let limit_us = limit.as_micros();
                let limit_micros = limit_us;
                let limit_microseconds = limit_us;
                let limit_ms = limit.as_millis();
                let limit_millis = limit_ms;
                let limit_milliseconds = limit_ms;
                let limit_s = limit.as_secs();
                let limit_secs = limit_s;
                let limit_seconds = limit_s;
                let limit_m = limit.as_secs() / 60;
                let limit_min = limit_m;
                let limit_minutes = limit_m;
                let limit_h = limit.as_secs() / 60 / 60;
                let limit_hours = limit_h;
                let limit_d = limit.as_secs() / 60 / 60 / 24;
                let limit_days = limit_d;

                #stmt
            }
            result
        }}
        .into(),
    )
    .unwrap();

    result.into_token_stream().into()
}
