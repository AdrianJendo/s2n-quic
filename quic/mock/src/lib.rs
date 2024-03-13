extern crate proc_macro;

use proc_macro::TokenStream;
use quote::__private::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemTrait, TraitItem, Ident};
use syn::parse::Parser;

#[proc_macro_attribute]
pub fn seamock(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input trait
    let input = parse_macro_input!(input as ItemTrait);
    let trait_name = Ident::new(&format!("Mock{}", &input.ident), Span::call_site());

    // Get the trait methods
    let trait_attrs = input.items.iter().filter_map(|item| {
        if let TraitItem::Method(method) = item {
            Some(
                Ident::new(&format!("times_{}", &method.sig.ident), method.sig.ident.span()),
            )
        } else {
            None
        }
    });

    let trait_methods = input.items.iter().filter_map(|item| {
        if let TraitItem::Method(method) = item {
            Some(
                Ident::new(&format!("expect_times_{}", &method.sig.ident), method.sig.ident.span()),
            )
        } else {
            None
        }
    });

    let x = trait_attrs.clone();

    // Generate the MockContext struct with RefCell fields for each method
    let mock_struct = quote! {
        struct #trait_name {
            #(
                #x: std::cell::RefCell<u8>,
            )*
        }
    };

    // Implement the trait for MockContext
    let trait_impl = quote! {
        impl #trait_name {
            pub fn new() -> Self {
                Self {
                    #(
                        #trait_attrs: RefCell::new(0),
                    )*
                }
            }
            #(
                fn #trait_methods(&self, times: u8) -> bool {
                    self.#trait_methods.borrow().clone() == times
                }
            )*
        }
    };

    // Combine the generated tokens
    let expanded = quote! {
        use core::cell::RefCell;
        #mock_struct
        #trait_impl
    };

    TokenStream::from(expanded)
}



// #[proc_macro_attribute]
// pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
//     println!("attr: \"{}\"", attr.to_string());
//     println!("item: \"{}\"", item.to_string());
//     item
// }
// #[proc_macro_attribute]
// pub fn add_field(_args: TokenStream, input: TokenStream) -> TokenStream  {
//
//     println!("attr: \"{}\"", _args.to_string());
//     println!("item: \"{}\"", input.to_string());
//
//     let args_parsed: Vec<String> = _args.to_string().split(", ").map(str::to_string).collect();
//
//     let mut ast = parse_macro_input!(input as DeriveInput);
//     match &mut ast.data {
//         syn::Data::Struct(ref mut struct_data) => {
//             match &mut struct_data.fields {
//                 syn::Fields::Named(fields) => {
//                     for field in args_parsed.iter() {
//                         // println!("{}", field);
//                         // quote!({ pub, field, : u8 });
//                         fields
//                             .named
//                             .push(syn::Field::parse_named.parse2(quote! { pub format!("{}", field): u8 }).unwrap());
//                     }
//                 }
//                 _ => {
//                     ()
//                 }
//             }
//
//             return quote! {
//                 #ast
//             }.into();
//         }
//         _ => panic!("`add_field` has to be used with structs "),
//     }
// }