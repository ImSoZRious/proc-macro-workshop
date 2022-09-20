use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Lit, LitStr, Meta, Type};

mod utils;
use utils::{get_first_ident, FieldsIter};
mod base;
use base::get_t_impl;
mod builder;
use builder::{get_struct, get_tbuilder_impl};

type Oaat = Vec<(Ident, LitStr, Type)>;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let oaat_fields = get_each(&input);

    if let Err(e) = oaat_fields {
        return proc_macro::TokenStream::from(e.to_compile_error());
    }

    let oaat_fields = oaat_fields.unwrap();

    let struct_stream = get_struct(&input);

    let impl_t_stream = get_t_impl(&input);

    let impl_tbuilder_stream = get_tbuilder_impl(&input, &oaat_fields);

    let expanded = quote! {
        #struct_stream

        #impl_t_stream

        #impl_tbuilder_stream
    };

    // Uncomment here to see expanded TokenStream
    // println!("{}", expanded);

    proc_macro::TokenStream::from(expanded)
}

fn get_each(input: &DeriveInput) -> syn::Result<Oaat> {
    let mut ret = vec![];

    for field in input.fields_iter() {
        let var_name = match field.ident {
            Some(ref x) => x,
            None => return Err(syn::Error::new(Span::call_site(), "Unnamed Field")),
        };

        for attr in field.attrs.iter() {
            let meta = attr.parse_meta();

            if let Err(e) = meta {
                return Err(e);
            }

            let meta = meta.unwrap();

            let attr_name = get_first_ident(&meta.path());

            if attr_name != Some("builder".to_owned()) {
                return Err(syn::Error::new(Span::call_site(), "Unknown attribute"));
            }

            match meta {
                Meta::List(ref list) => {
                    for x in list.nested.iter() {
                        match x {
                            syn::NestedMeta::Meta(ref sub_meta) => match sub_meta {
                                Meta::NameValue(ref name_value) => {
                                    let meta_name = get_first_ident(&name_value.path);

                                    if meta_name != Some("each".to_owned()) {
                                        return Err(syn::Error::new(
                                            meta.span(),
                                            "expected `builder(each = \"...\")`",
                                        ));
                                    }

                                    match name_value.lit {
                                        Lit::Str(ref x) => {
                                            ret.push((
                                                var_name.clone(),
                                                x.clone(),
                                                field.ty.clone(),
                                            ));
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    Ok(ret)
}
