use super::utils::{get_inside, is_option, is_vec, FieldsIter};
use super::Oaat;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, DeriveInput, Ident};

pub fn get_struct(input: &DeriveInput) -> TokenStream {
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

pub fn get_tbuilder_impl(input: &DeriveInput, oaat_fields: &Oaat) -> TokenStream {
    let builder_name = Ident::new(&format!("{}Builder", input.ident), Span::call_site());

    let setter = setter(input, &oaat_fields);

    let build_fn = build_fn(input);

    let oaat_fn = oaat_fn(input, &oaat_fields);

    quote! {
        impl #builder_name {
            #setter

            #build_fn

            #oaat_fn
        }
    }
}

fn setter(input: &DeriveInput, oaat_fields: &Oaat) -> TokenStream {
    let omitted_filed: Vec<_> = oaat_fields
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

fn oaat_fn(_input: &DeriveInput, fields: &Oaat) -> TokenStream {
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
