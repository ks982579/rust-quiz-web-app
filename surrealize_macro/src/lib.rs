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
