extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, ItemFn, ItemStruct};

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

fn process_key(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    let ident = &item.ident;

    let output = quote! {

        #[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
        #item

        impl Key for #ident {

            type Item = Field;
            type Metadata = KeyMetadata;
            type TypeId = usize;

            fn new(name: &str) -> Self {
                Self {
                    fields: Vec::new(),
                    metadata: KeyMetadata::default(),
                    type_id: type_id(name),
                }
            }

            fn metadata(&mut self, meta: KeyMetadata) -> &mut Self {
                self.metadata = KeyMetadata::with_key_code(meta, self.type_id);
                self
            }

            fn field(&mut self, f: Self::Item) -> &mut Self {
                self.fields.push(f);
                self
            }

            fn build(&self) -> Self {
                self.clone()
            }

            fn flatten(&self) -> Bytes {
                Bytes::from(
                    self.fields
                        .iter()
                        .map(|f| usize::to_le_bytes(f.type_id).to_vec())
                        .collect::<Vec<Vec<u8>>>()
                        .into_iter()
                        .flatten()
                        .collect::<Vec<u8>>(),
                )
            }

            fn get_metadata(&self) -> Self::Metadata {
                self.metadata.clone()
            }

            fn type_id(&self) -> Self::TypeId {
                self.type_id
            }
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

#[proc_macro_error]
#[proc_macro_attribute]
pub fn key(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    process_key(input)
}
