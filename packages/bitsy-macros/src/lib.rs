extern crate proc_macro;

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
            K: Key + std::cmp::Eq + std::hash::Hash + Send + Sync,
            D: Database + Send + Sync + 'static
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

        impl FoobarZoo for #ident {}

        impl Key for #ident {

            type Item = Field;
            type Metadata = KeyMetadata;
            type TypeId = usize;
            type Type = KeyType;

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
                let fields = Bytes::from(
                    self.fields
                        .iter()
                        .map(|f| usize::to_le_bytes(f.type_id).to_vec())
                        .collect::<Vec<Vec<u8>>>()
                        .into_iter()
                        .flatten()
                        .collect::<Vec<u8>>(),
                );

                let id = Bytes::from(usize::to_le_bytes(self.type_id).to_vec());
                Bytes::from([id, fields].concat())
                // Bytes::new()
            }

            fn get_metadata(&self) -> Self::Metadata {
                self.metadata.clone()
            }

            fn type_id(&self) -> Self::TypeId {
                self.type_id
            }
        }

        impl AsBytes for #ident {
            fn as_bytes(&self) -> &[u8] {
                <HttpKey as AsBytes>::as_bytes(self)
            }
        }

        impl #ident {
            pub fn from_input(key_name: &str, value: Input, extractors: Extractors) -> BitsyResult<Self> {
                let key = #ident::new(key_name);

                let ty_id = type_id(key_name);
                let fields = extractors.items.iter().map(|e| e.1.call(&value).expect("Failed to call on input.")).collect::<Vec<Field>>();

                Ok(#ident::from_fields(key_name, fields))
            }

            pub fn from_fields(key_name: &str, fields: Vec<Field>) -> Self {
                #ident{ fields, metadata: KeyMetadata::default(), type_id: type_id(key_name) }
            }
        }


        impl From<Bytes> for #ident {
            fn from(b: Bytes) -> #ident {
                let mut parts = b.chunks_exact(64);

                let key_ty_id = parts.next().unwrap();
                let key_name = std::str::from_utf8(&key_ty_id).expect("Bad key.");

                // let ty_id = usize::from_le_bytes(parts[0]);

                let key = #ident::new(&key_name);

                key
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
pub fn key(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    process_key(input)
}
