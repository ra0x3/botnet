extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, ItemFn};

fn process_task(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let ident = format_ident!("{}Task", parse_macro_input!(attrs as Ident));

    let output = quote! {

        struct #ident{}

        #[async_trait::async_trait]
        impl<K, D> Task<K, D> for #ident
        where
            K: DatabaseKey + 'static,
            D: Database + Send + Sync + 'static
        {
            #[allow(unused)]
            #func
        }

    };

    TokenStream::from(output)
}

fn process_evaluator(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let ident = format_ident!("{}Evaluator", parse_macro_input!(attrs as Ident));

    let output = quote! {

        struct #ident{}

        #[async_trait::async_trait]
        impl Evaluator for #ident {
            #[allow(unused)]
            #func
        }

    };

    TokenStream::from(output)
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn task(attrs: TokenStream, input: TokenStream) -> TokenStream {
    process_task(attrs, input)
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn evaluator(attrs: TokenStream, input: TokenStream) -> TokenStream {
    process_evaluator(attrs, input)
}
