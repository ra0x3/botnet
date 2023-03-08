extern crate proc_macro;

use bitsy_utils::type_id;
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

            const NAME: &'static str = #name;
            const TYPE_ID: usize = #ty_id;

            type Item = Field;
            type Metadata = KeyMetadata;

            fn builder() -> Self {
                Self {
                    fields: Vec::new(),
                    metadata: KeyMetadata::default(),
                }
            }

            fn metadata(&mut self, meta: KeyMetadata) -> &mut Self {
                self.metadata = KeyMetadata::with_key_code(meta, Self::TYPE_ID);
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

                let id = Bytes::from(usize::to_le_bytes(Self::TYPE_ID).to_vec());
                Bytes::from([id, fields].concat())
                // Bytes::new()
            }

            fn get_metadata(&self) -> Self::Metadata {
                self.metadata.clone()
            }

            fn type_id(&self) -> usize {
                Self::TYPE_ID
            }

            fn name(&self) -> &'static str {
                Self::NAME
            }
        }

        impl AsBytes for #ident {
            fn as_bytes(&self) -> &[u8] {
                <#ident as AsBytes>::as_bytes(self)
            }
        }

        impl #ident {
            pub fn from_input(value: Input, extractors: &Extractors, metadata: &Metadata) -> BitsyResult<Self> {
                let meta = metadata.get(&Self::TYPE_ID);
                let fields = extractors.items.iter().map(|e| e.1.call(&value).expect("Failed to call on input.")).collect::<Vec<Field>>();

                // TODO: use builder pattern
                Ok(#ident{ fields, metadata: meta.to_owned() })
            }

            pub fn from_bytes(b: Bytes, metadata: &Metadata) -> BitsyResult<#ident> {
                let mut parts = b.chunks_exact(64);

                let key_ty_id = parts.next().unwrap();
                let meta = metadata.get(&Self::TYPE_ID);

                // let ty_id = usize::from_le_bytes(parts[0]);

                Ok(#ident::builder())
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
