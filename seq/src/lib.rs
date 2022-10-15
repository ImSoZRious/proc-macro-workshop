mod seq_input;

use seq_input::SeqInput;

use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn seq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as SeqInput);

    // println!("{:?}", input);

    let expanded = quote! {
        #input
    };

    proc_macro::TokenStream::from(expanded)
}
