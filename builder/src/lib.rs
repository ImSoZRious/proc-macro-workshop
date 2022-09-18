use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Lit, LitStr, Meta, Type};

mod utils;
use utils::{get_first_ident, get_inside, is_option, is_vec, FieldsIter};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let oota_fields = get_each(&input);

    if let Err(e) = oota_fields {
        return proc_macro::TokenStream::from(e.to_compile_error());
    }

    let oota_fields = oota_fields.unwrap();

    let struct_stream = get_struct(&input);

    let impl_t_stream = get_t_impl(&input);

    let impl_tbuilder_stream = get_tbuilder_impl(&input, &oota_fields);

    let expanded = quote! {
        #struct_stream

        #impl_t_stream

        #impl_tbuilder_stream
    };

    // Uncomment here to see expanded TokenStream
    // println!("{}", expanded);

    proc_macro::TokenStream::from(expanded)
}

fn get_struct(input: &DeriveInput) -> TokenStream {
    let struct_name = Ident::new(&format!("{}Builder", input.ident), Span::call_site());

    let recurse = input.fields_iter().map(|attr| {
        let name = &attr.ident;
        let ty = &attr.ty;

        if is_option(ty) || is_vec(ty) {
            quote_spanned! {attr.span()=>
                #name: #ty
            }
        } else {
            quote_spanned! {attr.span()=>
                #name: std::option::Option<#ty>
            }
        }
    });

    let fields = quote! {
        #(#recurse,)*
    };

    quote_spanned! {fields.span()=>
        pub struct #struct_name {
            #fields
        }
    }
}

fn get_t_impl(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;

    let builder = builder(input);

    quote! {
        impl #name {
            #builder
        }
    }
}

fn get_tbuilder_impl(input: &DeriveInput, oota_fields: &Vec<(Ident, LitStr, Type)>) -> TokenStream {
    let builder_name = Ident::new(&format!("{}Builder", input.ident), Span::call_site());

    let setter = setter(input, &oota_fields);

    let build_fn = build_fn(input);

    let oota_fn = oota_fn(input, &oota_fields);

    quote! {
        impl #builder_name {
            #setter

            #build_fn

            #oota_fn
        }
    }
}

fn builder(input: &DeriveInput) -> TokenStream {
    let builder_name = Ident::new(&format!("{}Builder", input.ident), Span::call_site());

    let recurse = input.fields_iter().map(|attr| {
        let name = &attr.ident;
        let ty = &attr.ty;

        if is_vec(ty) {
            quote_spanned! {attr.span()=>
                #name: Vec::new()
            }
        } else {
            quote_spanned! {attr.span()=>
                #name: std::option::Option::None
            }
        }
    });

    let fields = quote! {
        #(#recurse,)*
    };

    quote! {
        pub fn builder() -> #builder_name {
            #builder_name {
                #fields
            }
        }
    }
}

fn setter(input: &DeriveInput, oota_fields: &Vec<(Ident, LitStr, Type)>) -> TokenStream {
    let omitted_filed: Vec<_> = oota_fields
        .iter()
        .map(|x| Ident::new(x.1.value().as_ref(), Span::call_site()))
        .collect();

    let recurse = input.fields_iter().filter_map(|x| {
        let name = match x.ident {
            Some(ref x) => x,
            None => return None,
        };

        if omitted_filed.contains(name) {
            return None;
        }

        let ty = &x.ty;

        if is_option(ty) {
            let inside_ty = get_inside(ty);

            Some(quote_spanned! {x.span()=>
                pub fn #name (&mut self, #name: #inside_ty) -> &mut Self {
                    self.#name = std::option::Option::Some(#name);
                    self
                }
            })
        } else if is_vec(ty) {
            Some(quote_spanned! {x.span()=>
                pub fn #name (&mut self, #name: #ty) -> &mut Self {
                    self.#name = #name;
                    self
                }
            })
        } else {
            Some(quote_spanned! {x.span()=>
                pub fn #name (&mut self, #name: #ty) -> &mut Self {
                    self.#name = std::option::Option::Some(#name);
                    self
                }
            })
        }
    });

    quote! {
        #(#recurse)*
    }
}

fn build_fn(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;

    let recurse = input.fields_iter().filter_map(|x| {
        let name = &x.ident;

        let ty = &x.ty;

        if !is_option(ty) && !is_vec(ty) {
            return Some(quote_spanned! {x.span()=>
                self.#name.is_none()
            });
        }
        None
    });

    let check_exist = quote! {
        if false #(||#recurse)*
        {
            return std::result::Result::Err(std::boxed::Box::<dyn std::error::Error>::from("Unspecify field".to_owned()));
        }
    };

    let recurse = input.fields_iter().map(|x| {
        let name = &x.ident;
        let ty = &x.ty;

        if is_option(ty) || is_vec(ty) {
            quote_spanned! {x.span()=>
                #name: self.#name.clone()
            }
        } else {
            quote_spanned! {x.span()=>
                #name: self.#name.clone().unwrap()
            }
        }
    });

    let fields = quote! {
        #(#recurse,)*
    };

    quote! {
        pub fn build(&mut self) -> std::result::Result<#name, std::boxed::Box<dyn std::error::Error>> {
            #check_exist

            std::result::Result::Ok(#name {
                #fields
            })
        }
    }
}

fn oota_fn(_input: &DeriveInput, fields: &Vec<(Ident, LitStr, Type)>) -> TokenStream {
    let recurse = fields.iter().map(|x| {
        let var_name = &x.0;
        let fn_name = Ident::new(&x.1.value().as_str(), Span::call_site());
        let ty = get_inside(&x.2);

        quote! {
            pub fn #fn_name (&mut self, value: #ty) -> &mut Self {
                self.#var_name.push(value);

                self
            }
        }
    });

    quote! {
        #(#recurse)*
    }
}

fn get_each(input: &DeriveInput) -> syn::Result<Vec<(Ident, LitStr, Type)>> {
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
