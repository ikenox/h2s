use darling::{FromDeriveInput, FromField, FromMeta};
use kuchiki::Selectors;
use proc_macro::TokenStream;
use quote::__private::ext::RepToTokensExt;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, PathArguments, PathSegment, Type, TypePath};

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
        ty: syn::Type,
        ident: Option<syn::Ident>,
        select: Option<String>,
        attr: Option<String>,
        #[darling(default)]
        text: bool,
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
                .map(
                    |H2sFieldReceiver {
                         ty,
                         ident,
                         select,
                         attr,
                         text,
                     }| {
                        // all fields must be named
                        let ident = ident
                            .as_ref()
                            .expect(&format!("all struct fields for h2s must be named."));
                        // check selector validity at compile time
                        if let Some(selector) = select {
                            Selectors::compile(selector)
                                .expect(&format!("invalid css selector: `{}`", selector));
                        }

                        let n = match select {
                            Some(selector) => {
                                quote!( ::h2s::select(input, & #selector .to_string())? )
                            }
                            None => quote!(input.clone()),
                        };
                        let extractor = if let Some(attr) = attr {
                            quote!(::h2s::AttributeExtractor{attr: #attr .to_string()})
                        } else if *text {
                            quote!(::h2s::TextContentExtractor)
                        } else {
                            quote!(::h2s::StructExtractor::new())
                        };

                        // FIXME naive implementation to build type-hint quote
                        //       Or, if rust compiler is improved in future, can this annotation be removed?
                        let type_hint = match ty {
                            Type::Path(p) => p
                                .path
                                .segments
                                .first()
                                .map(|s| match &s.arguments {
                                    PathArguments::AngleBracketed(_) => {
                                        let ident = &s.ident;
                                        quote!(#ident<N>)
                                    }
                                    PathArguments::Parenthesized(_) => quote!(_),
                                    PathArguments::None => {
                                        quote!(N)
                                    }
                                })
                                .unwrap_or_else(|| quote!(_)),
                            _ => quote!(_),
                        };

                        quote!(#ident: ::h2s::extract::<#type_hint,_,_>(#n, #extractor)
                            .map_err(|a|a)?)
                    },
                );

            tokens.extend(quote! {
                impl ::h2s::FromHtml for #ident {
                    fn from_html<N: ::h2s::HtmlNodeRef>(
                        input: &N,
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
