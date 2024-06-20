use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let builder_name = format_ident!("{}Builder", name);
    let (builder_def, builder_init) = convert_to_option(&input.data);

    let expanded = quote! {
        pub struct #builder_name {
            #builder_def
        }
        impl #name {
            pub fn builder() -> #builder_name{
                #builder_name {
                    #builder_init
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn convert_to_option(data: &Data) -> (TokenStream, TokenStream) {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let def = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    quote! {
                        #name: Option<#ty>
                    }
                });
                let init = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! {
                        #name: None
                    }
                });
                (
                    quote! {
                        #(#def),*
                    },
                    quote! {
                        #(#init),*
                    },
                )
            }
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
