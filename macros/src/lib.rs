use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Type, TypePath};

extern crate proc_macro;

trait IsOption {
    fn is_option(&self) -> bool;

    fn inner_type(&self) -> Option<&Type>;
}

impl IsOption for TypePath {
    fn is_option(&self) -> bool {
        if let Some(syn::PathSegment {
            ident,
            arguments:
                syn::PathArguments::AngleBracketed(
                    syn::AngleBracketedGenericArguments { args, .. },
                ),
        }) = self.path.segments.first()
        {
            if ident == "Option" {
                if let syn::GenericArgument::Type(_) = args.first().unwrap() {
                    return true;
                }
            }
        }
        false
    }

    fn inner_type(&self) -> Option<&Type> {
        if !self.is_option() {
            return None;
        }
        if let Some(syn::PathSegment {
            ident,
            arguments:
                syn::PathArguments::AngleBracketed(
                    syn::AngleBracketedGenericArguments { args, .. },
                ),
        }) = self.path.segments.first()
        {
            if ident == "Option" {
                if let syn::GenericArgument::Type(ty) = args.first().unwrap() {
                    return Some(ty);
                }
            }
        }
        None
    }
}

impl IsOption for Type {
    fn is_option(&self) -> bool {
        if let syn::Type::Path(path) = self {
            return path.is_option();
        }
        false
    }

    fn inner_type(&self) -> Option<&Type> {
        if let syn::Type::Path(path) = self {
            return path.inner_type();
        }
        None
    }
}

#[proc_macro_derive(Builder)]
pub fn derive_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let strct = parse_macro_input!(input as syn::ItemStruct);
    let strct_name = &strct.ident;
    let fields = match &strct.fields {
        syn::Fields::Named(ref fields) => {
            fields.named.iter().collect::<Vec<_>>()
        }
        _ => panic!("Only named fields are supported"),
    };
    let builder_name =
        Ident::new(&format!("{}Builder", strct.ident), strct.ident.span());
    let builder_fields = (&fields)
        .iter()
        .map(|field| {
            let name = &field.ident;
            let ty = &field.ty;
            // if ty is `Option`
            if ty.is_option() {
                return quote! {
                    #name: #ty
                };
            }
            return quote! {
                #name: Option<#ty>
            };
        })
        .collect::<Vec<_>>();
    let builder_init_fields = fields
        .iter()
        .map(|field| {
            let name = &field.ident;
            return quote! {
                #name: None
            };
        })
        .collect::<Vec<_>>();
    let builder_methods = fields
        .iter()
        .map(|field| {
            let name = &field.ident;
            let ty = &field.ty;
            if let Some(inner_ty) = ty.inner_type() {
                return quote! {
                    pub fn #name(&mut self, #name: #inner_ty) -> &mut Self {
                        self.#name = Some(#name);
                        self
                    }
                };
            } else {
                return quote! {
                    pub fn #name(&mut self, #name: #ty) -> &mut Self {
                        self.#name = Some(#name);
                        self
                    }
                };
            }
        })
        .collect::<Vec<_>>();
    let builder_build_fields = fields
        .iter()
        .map(|field| {
            let name = &field.ident;
            let ty = &field.ty;
            if ty.is_option() {
                return quote! {
                    #name: self.#name.take()
                };
            }
            let err_msg = format!("{} is not set", name.as_ref().unwrap());
            return quote! {
                #name: self.#name.take().expect(#err_msg)
            };
        })
        .collect::<Vec<_>>();
    quote! {
        impl #strct_name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#builder_init_fields),*
                }
            }
        }

        pub struct #builder_name {
            #(#builder_fields),*
        }

        impl #builder_name {
            #(#builder_methods)*

            pub fn build(&mut self) -> #strct_name {
                #strct_name {
                    #(#builder_build_fields),*
                }
            }
        }
    }
    .into()
}
