//! surrealize_macro/src/lib.rs
//! The idea is when you create a struct to store data in SurrealDB, you use that very
//! struct to pull data back out. But you'll need the ID sometimes to know which
//! record you want. This macro recreates each struct, adding that "id" field so the
//! user doesn't have to explicitly create 2 structs.
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Surrealize)]
pub fn surrealize_macro(input: TokenStream) -> TokenStream {
    // Parsing input tokens into syntax tree
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    // Getting Struct Name
    let struct_name: Ident = input.ident.clone();

    // Create new struct name prepending "Surreal"
    let surreal_name: Ident = Ident::new(&format!("Surreal{}", struct_name), struct_name.span());

    // Getting fields of struct
    let struct_fields = match &input.data {
        syn::Data::Struct(ref data) => &data.fields,
        _ => panic!("`Surrealize` can only be derived for structs"),
    };

    // Generate fields for new struct - does not include "id" yet.
    let surreal_fields = struct_fields.iter().map(|fld| {
        let name = &fld.ident;
        let ty = &fld.ty; // type
        quote! { #name: #ty }
    });

    let field_names = struct_fields.iter().map(|f| &f.ident);

    let expanded: proc_macro2::TokenStream = quote! {
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        pub struct #surreal_name {
            pub id: surrealdb::sql::Thing,
            #(pub #surreal_fields,)*
        }

        impl From<#surreal_name> for #struct_name {
            fn from(arg: #surreal_name) -> Self {
                Self{
                    #(#field_names: arg.#field_names,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
