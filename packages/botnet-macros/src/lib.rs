extern crate proc_macro;

use crate::parser::BotnetMainArgs;
use botnet_core::config::*;
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
            D: Store + Send + Sync + 'static
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

        impl extractor::Extractor for #ident {
            fn extract(&self, key: &str, input: &input::Input) -> BotnetResult<field::ExtractedField> {
                #block
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
    let pathstr = path.clone().to_str().unwrap().to_string();

    match canonicalize(path) {
        Ok(path) => {
            let config: BotnetConfig<IPUAEntityCounter, KAnonimity, CliffDetector> =
                BotnetConfig::from(path);
            let mut tokens = quote! {
                let mut extractors = extractor::Extractors::new();
            };

            let ext_idents = vec![
                format_ident!("IPUAEntityCounter"),
                format_ident!("KAnonimity"),
                format_ident!("CliffDetector"),
            ];

            for k in config.keys {
                tokens = quote! {
                    #tokens
                    let mut field_exts = extractor::FieldExtractors::new();
                };

                for f in &k.fields {
                    let ext_name = format_ident!("{}", f.extractor);
                    let key = f.key.to_string();
                    tokens = quote! {
                        #tokens
                        field_exts.insert(#key.to_string(), extractor::FieldExtractor {
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

                use botnet::core::models::extractor::*;

                let p = format!("{}", #pathstr);
                let config = BotnetConfig::from(PathBuf::from(&p));
                let key_meta = config.keys.iter().map(|k| {
                    let field_meta = k.fields.iter().map(|f| field::FieldMetadata::new(&f.name, &f.name, f.description.as_ref())).collect::<Vec<field::FieldMetadata>>();
                    (k.type_id(), key::BotnetKeyMetadata::from((k.type_id(), k.name.as_str(), field_meta)))
                });
                let metadata = Metadata::from(key_meta);
                let db = Some(InMemory::default());


                #tokens

                let context = BotnetContext::<#(#ext_idents),*>::new(metadata, extractors, db, config);
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

fn process_botnet_test(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let manifest = parse_macro_input!(attrs as BotnetMainArgs);
    let mut func = parse_macro_input!(input as ItemFn);

    let BotnetMainArgs { path } = manifest;

    let path = canonicalize(path).expect("Failed to canonicalize path.");
    let pathstr = path.clone().to_str().unwrap().to_string();

    match canonicalize(path) {
        Ok(path) => {
            let config: BotnetConfig<IPUAEntityCounter, KAnonimity, CliffDetector> =
                BotnetConfig::from(path);
            let mut tokens = quote! {
                let mut extractors = extractor::Extractors::new();
            };

            let ext_idents = vec![
                format_ident!("{}", config.plan.entity.class()),
                format_ident!("{}", config.plan.anonimity.class()),
                format_ident!("{}", config.plan.limiter.class()),
            ];

            for k in config.keys {
                tokens = quote! {
                    #tokens
                    let mut field_exts = field::FieldExtractors::new();
                };
                for f in &k.fields {
                    let ext_name = format_ident!("{}", f.extractor);
                    let key = f.key.to_string();
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

                use botnet_core::models::extractor::*;

                let p = format!("{}", #pathstr);
                let config = BotnetConfig::from(PathBuf::from(&p));
                let key_meta = config.keys.iter().map(|k| {
                    let field_meta = k.fields.iter().map(|f| field::FieldMetadata::new(&f.name, &f.name, f.description.as_ref())).collect::<Vec<field::FieldMetadata>>();
                    (k.type_id(), key::BotnetKeyMetadata::from((k.type_id(), k.name.as_str(), field_meta)))
                });
                let metadata = Metadata::from(key_meta);
                let db = Some(InMemory::default());

                #tokens

                let context = BotnetContext::<#(#ext_idents),*>::new(metadata, extractors, db, config);
            };

            let block: Block = parse_quote!({
                #injection
            });

            let Block { ref mut stmts, .. } = *func.block;

            for stmt in block.stmts.into_iter().rev() {
                stmts.insert(0, stmt);
            }

            let block = &func.block;
            let name = &func.sig.ident;

            let output = quote! {

                #[test]
                fn #name() {
                    #block
                }
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

#[proc_macro_error]
#[proc_macro_attribute]
pub fn botest(attrs: TokenStream, input: TokenStream) -> TokenStream {
    process_botnet_test(attrs, input)
}
