use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let builder_ident = format_ident!("{}Builder", name);
    let builder_def = convert_to_option(
        &input.data,
        |f| {
            let name = &f.ident;
            let ty = &f.ty;
            quote! {
                #name: Option<#ty>
            }
        },
        true,
    );

    let builder_init = convert_to_option(
        &input.data,
        |f| {
            let name = &f.ident;
            quote! {
                #name: None
            }
        },
        true,
    );

    let builder_impl = convert_to_option(
        &input.data,
        |f| {
            let name = &f.ident;
            let ty = &f.ty;
            quote! {
                pub fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = Some(#name);
                    self
                }
            }
        },
        false,
    );

    let expanded = quote! {
        pub struct #builder_ident {
            #builder_def
        }
        impl #name {
            pub fn builder() -> #builder_ident{
                #builder_ident {
                    #builder_init
                }
            }
        }
        impl #builder_ident {
            #builder_impl
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn convert_to_option(
    data: &Data,
    f: impl FnMut(&Field) -> TokenStream,
    comma: bool,
) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(f);
                if comma {
                    quote! {
                        #(#recurse),*
                    }
                } else {
                    quote! {
                        #(#recurse)*
                    }
                }
            }
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
