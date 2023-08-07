extern crate proc_macro;

use crate::parser::BotnetMainArgs;
use botnet_core::config::BotnetConfig;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{format_ident, quote};
use std::fs::canonicalize;
use syn::{parse_macro_input, parse_quote, Block, Ident, ItemFn};

pub(crate) mod parser;

fn process_task(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let ident = format_ident!("{}Task", parse_macro_input!(attrs as Ident));

    let output = quote! {

        struct #ident;

        #[async_trait::async_trait]
        impl<K, D> Task<K, D> for #ident
        where
            K: DatabaseKey + 'static,
            D: Database + Send + Sync + 'static
        {
            #func
        }

    };

    TokenStream::from(output)
}

fn process_evaluator(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let ident = format_ident!("{}Evaluator", parse_macro_input!(attrs as Ident));

    let output = quote! {

        struct #ident;

        #[async_trait::async_trait]
        impl Evaluator for #ident {
            #func
        }

    };

    TokenStream::from(output)
}

fn process_extractor(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let mut ident = format_ident!("{}", parse_macro_input!(attrs as Ident)).to_string();
    let ident = format!("{}{ident}", ident.remove(0).to_uppercase());
    let ident = format_ident!("{ident}Extractor");

    let block = &func.block;

    let output = quote! {

        struct #ident;

        impl Extractor for #ident {
            fn extract(&self, key: &str, input: &Input) -> BotnetResult<TransparentField> {
                #block
            }
        }

        impl From<String> for #ident {
            fn from(_: String) -> Self {
                Self
            }
        }

    };

    TokenStream::from(output)
}

fn process_main(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as ItemFn);
    let manifest = parse_macro_input!(attrs as BotnetMainArgs);

    let BotnetMainArgs { path } = manifest;

    let path = canonicalize(path).expect("Failed to canonicalize path.");

    match canonicalize(path) {
        Ok(path) => {
            let config = BotnetConfig::from(path);
            let mut tokens = quote! {
                let mut extractors = Extractors::new();
            };

            for k in config.keys {
                tokens = quote! {
                    #tokens
                    let mut field_exts = FieldExtractors::new();
                };
                for f in &k.fields {
                    let ext_name = format_ident!("{}", f.extractor);
                    let key = format!("\"{}\"", f.key);
                    tokens = quote! {
                        #tokens
                        field_exts.insert(#key.to_string(), FieldExtractor {
                            key: #key.to_string(),
                            func: Box::new(#ext_name) as Box<dyn Extractor>,
                        });
                    }
                }

                let ty_id = k.type_id();
                tokens = quote! {
                    #tokens
                    extractors.insert(#ty_id, field_exts);
                }
            }

            let injection = quote! {

                let opts = Args::parse();
                let config = BotnetConfig::from(opts.config.unwrap_or_default());
                let keys = config.keys.iter().map(|k| (k.type_id(), BotnetKey::from(k))).collect::<HashMap<usize, BotnetKey>>();

                let key_meta = config.keys.iter().map(|k| (k.type_id(), BotnetKeyMetadata::new(&k.name)));
                let metadata = Metadata::from(key_meta);
                let db = Some(InMemory::default());

                #tokens

                let context = BotnetContext::new(keys, metadata, extractors, db, config);
            };

            let block: Block = parse_quote!({
                #injection
            });

            let Block { ref mut stmts, .. } = *func.block;

            for stmt in block.stmts.into_iter().rev() {
                stmts.insert(0, stmt);
            }

            let output = quote! {

                #[tokio::main]
                #func
            };

            TokenStream::from(output)
        }
        Err(e) => {
            proc_macro_error::abort_call_site!("Failed to canonicalize path: {:?}", e);
        }
    }
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

#[proc_macro_error]
#[proc_macro_attribute]
pub fn evaluator(attrs: TokenStream, input: TokenStream) -> TokenStream {
    process_evaluator(attrs, input)
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn main(attrs: TokenStream, input: TokenStream) -> TokenStream {
    process_main(attrs, input)
}
