extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, ItemFn};

fn process_extractor(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let ident = format_ident!("{}Extractor", parse_macro_input!(attrs as Ident));

    let output = quote! {
        struct #ident{}

        impl Extractor for #ident {
            #func
        }
    };

    TokenStream::from(output)
}

fn process_task(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let ident = format_ident!("{}Task", parse_macro_input!(attrs as Ident));

    let output = quote! {

        struct #ident{}

        #[async_trait::async_trait]
        impl<K, D> Task<K, D> for #ident
        where
            K: Key + std::cmp::Eq + std::hash::Hash + Send + Sync,
            D: Database<K> + Send + Sync + 'static
        {
            type Database = D;
            #func
        }

    };

    TokenStream::from(output)
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn extractor(attrs: TokenStream, input: TokenStream) -> TokenStream {
    process_extractor(attrs, input)
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn task(attrs: TokenStream, input: TokenStream) -> TokenStream {
    process_task(attrs, input)
}
