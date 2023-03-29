extern crate proc_macro;

use botnet_utils::type_id;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, ItemFn, ItemStruct};

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

fn process_key(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    let ident = &item.ident;
    let name = ident.to_string();
    let ty_id = type_id(&*name);

    let output = quote! {

        #[derive(Debug, PartialEq, Eq, Hash, Clone)]
        #item

        impl DatabaseKey for #ident {}

        impl Key for #ident {

            fn flatten(&self) -> Bytes {
                let fields = Bytes::from(
                    self.fields
                        .iter()
                        .map(|f| usize::to_le_bytes(f.type_id).to_vec())
                        .collect::<Vec<Vec<u8>>>()
                        .into_iter()
                        .flatten()
                        .collect::<Vec<u8>>(),
                );

                let id = Bytes::from(usize::to_le_bytes(#ty_id).to_vec());
                Bytes::from([id, fields].concat())
            }

            fn get_metadata(&self) -> KeyMetadata {
                self.metadata.clone()
            }

            fn type_id(&self) -> usize {
                #ty_id
            }

            fn name(&self) -> &'static str {
                #name
            }
        }

        impl AsBytes for #ident {
            fn as_bytes(&self) -> &[u8] {
                <#ident as AsBytes>::as_bytes(self)
            }
        }

        impl #ident {
            pub fn from_input(value: Input, extractors: &KeyExtractors, metadata: &Metadata) -> BotnetResult<Self> {
                let meta = metadata.get(&#ty_id);
                let fields = extractors.items.iter().map(|e| e.1.call(&value).expect("Failed to call on input.")).collect::<Vec<Field>>();

                // TODO: use builder pattern
                Ok(#ident{ fields, metadata: meta.to_owned() })
            }

            pub fn from_bytes(b: Bytes, metadata: &Metadata) -> BotnetResult<#ident> {
                let mut parts = b.chunks_exact(64);

                let key_ty_id = parts.next().unwrap();
                let meta = metadata.get(&#ty_id);

                // TODO: finish
                Ok(#ident { metadata: meta.to_owned(), fields: Vec::new() })
            }
        }

        impl Default for #ident {
            fn default() -> Self {
                Self {
                    fields: Vec::new(),
                    metadata: KeyMetadata::default(),
                }
            }
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

#[proc_macro_error]
#[proc_macro_attribute]
pub fn key(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    process_key(input)
}
