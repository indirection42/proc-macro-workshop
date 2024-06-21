use proc_macro2::Span;
use syn::{Error, Item};

#[proc_macro_attribute]
pub fn sorted(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as Item);

    match input {
        Item::Enum(_) => {}
        _ => {
            return Error::new(Span::call_site(), "expected enum or match expression")
                .to_compile_error()
                .into();
        }
    }

    let output = quote::quote! {
        #input
    };
    proc_macro::TokenStream::from(output)
}
