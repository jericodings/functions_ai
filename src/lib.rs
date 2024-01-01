extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

/// This attribute macro converts a Rust function into a function that returns its own definition as a string.
///
/// The generated function has the same signature as the original function, but instead of executing the original code,
/// it returns a string containing the entire source code of the function, including doc strings and other attributes.
///
/// This can be useful for sending the function's code to a large language model for further processing.
///
/// # Usage
///
/// Simply annotate your function with `#[function_ai]`.
///
/// ```ignore
/// #[function_ai]
/// fn example_function(arg: i32) -> i32 {
///     arg * 2
/// }
/// ```
///
/// When you call `example_function()`, instead of returning `arg * 2`, it will return a string containing the source code
/// of the `example_function`.
///
/// # Limitations
///
/// The `#[function_ai]` macro currently does not support functions with complex control flow like loops or conditionals.
/// It only supports simple function definitions.
///
/// # Example
///
/// ```ignore
/// #[function_ai]
/// fn add(a: i32, b: i32) -> i32 {
///     a + b
/// }
///
/// assert_eq!(add(2, 3), "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}");
/// ```
#[proc_macro_attribute]
pub fn function_ai(_attr: TokenStream, item: TokenStream) -> TokenStream {

    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);

    let function_str: String = format!("{}", input_fn.to_token_stream());

    let fn_ident: proc_macro2::Ident = input_fn.sig.ident;
    let fn_inputs: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma> = input_fn.sig.inputs;
    let fn_generics: syn::Generics = input_fn.sig.generics;

    let output: proc_macro2::TokenStream = quote! {
        pub fn #fn_ident #fn_generics(#fn_inputs) -> &'static str {
            #function_str
        }
    };

    output.into()
}