use super::utils::{is_vec, FieldsIter};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, DeriveInput, Ident};

pub fn get_t_impl(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;

    let builder = builder(input);

    quote! {
        impl #name {
            #builder
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
