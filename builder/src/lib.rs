use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Type};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let builder_ident = format_ident!("{}Builder", name);
    let builder_def = map_struct_fields(
        &input.data,
        |f| {
            let name = &f.ident;
            let ty = &f.ty;
            if let Type::Path(type_path) = ty {
                if let Some(segment) = type_path.path.segments.first() {
                    if segment.ident == "Option" {
                        return quote! {
                            #name: #ty
                        };
                    }
                }
            }
            quote! {
                #name: std::option::Option<#ty>
            }
        },
        true,
    );

    let builder_init = map_struct_fields(
        &input.data,
        |f| {
            let name = &f.ident;
            quote! {
                #name: std::option::Option::None
            }
        },
        true,
    );

    let builder_setters = map_struct_fields(
        &input.data,
        |f| {
            let name = &f.ident;
            let ty = &f.ty;
            if let Type::Path(type_path) = ty {
                if let Some(segment) = type_path.path.segments.first() {
                    if segment.ident == "Option" {
                        if let syn::PathArguments::AngleBracketed(angle_bracketed_args) =
                            &segment.arguments
                        {
                            if let Some(syn::GenericArgument::Type(inner_ty)) =
                                angle_bracketed_args.args.first()
                            {
                                return quote! {
                                    pub fn #name(&mut self, #name: #inner_ty) -> &mut Self {
                                        self.#name = std::option::Option::Some(#name);
                                        self
                                    }
                                };
                            }
                        }
                    }
                }
            }
            quote! {
                pub fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = std::option::Option::Some(#name);
                    self
                }
            }
        },
        false,
    );

    let builder_build = map_struct_fields(
        &input.data,
        |f| {
            let name = &f.ident;
            if let Type::Path(type_path) = &f.ty {
                if let Some(segment) = type_path.path.segments.first() {
                    if segment.ident == "Option" {
                        return quote! {
                            #name: self.#name.clone()
                        };
                    }
                }
            }
            quote! {
                #name: self.#name.clone().ok_or("Field not set")?
            }
        },
        true,
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
            #builder_setters
            pub fn build(&mut self) -> std::result::Result<#name, std::boxed::Box<dyn std::error::Error>> {
                std::result::Result::Ok(#name {
                    #builder_build
                })
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn map_struct_fields(
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
