use darling::{FromDeriveInput, FromField};
use h2s_core::ExtractionError::Unexpected;
use h2s_core::TextExtractionMethod;
use h2s_core::TextExtractionMethod::{Attribute, TextContent};
use kuchiki::Selectors;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

#[proc_macro_derive(FromHtml, attributes(h2s))]
pub fn derive(input: TokenStream) -> TokenStream {
    #[derive(Debug, FromDeriveInput)]
    #[darling(attributes(h2s), supports(struct_any))]
    pub struct H2sStructReceiver {
        ident: syn::Ident,
        data: darling::ast::Data<(), H2sFieldReceiver>,
    }
    #[derive(Debug, FromField)]
    #[darling(attributes(h2s))]
    pub struct H2sFieldReceiver {
        ident: Option<syn::Ident>,
        select: Option<String>,
        attr: Option<String>,
    }

    impl ToTokens for H2sStructReceiver {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            let Self {
                ref ident,
                ref data,
            } = *self;

            let field_and_values = data
                .as_ref()
                .take_struct()
                .expect(
                    format!(
                        "{} should be struct because it is deriving H2s",
                        ident.to_string()
                    )
                    .as_str(),
                )
                .fields
                .into_iter()
                .enumerate()
                .map(
                    |(
                        i,
                        H2sFieldReceiver {
                            ident,
                            select,
                            attr,
                        },
                    )| {
                        // all fields must be named
                        let ident = ident
                            .as_ref()
                            .expect(&format!("all struct fields for h2s must be named."));
                        // check selector validity at compile time
                        if let Some(selector) = select {
                            Selectors::compile(selector)
                                .expect(&format!("invalid css selector: `{}`", selector));
                        }

                        let selector = match select {
                            Some(selector) => quote!(Some(#selector)),
                            None => quote!(None),
                        };
                        let attr = match attr {
                            Some(attr) => quote!(Some(#attr)),
                            None => quote!(None),
                        };

                        quote!(
                            #ident: ::h2s::macro_utils::build_struct_field_value(
                                node,
                                #selector,
                                &( ::h2s::ArgBuilder{ attr: #attr } )
                            )?
                        )
                    },
                );

            tokens.extend(quote! {
                impl ::h2s::FromHtml for #ident {
                    type Source = ::kuchiki::NodeRef;
                    type Args = ();
                    fn extract_from(
                        node: &Self::Source,
                        args: &Self::Args,
                    ) -> Result<Self, ::h2s::ExtractionError> {
                        Ok(Self{
                            #(#field_and_values),*
                        })
                    }
                }
            });
        }
    }

    let struct_receiver: H2sStructReceiver =
        H2sStructReceiver::from_derive_input(&parse_macro_input!(input)).unwrap();
    quote!(#struct_receiver).into()
}
