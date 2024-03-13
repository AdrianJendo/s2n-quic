extern crate proc_macro;

use proc_macro::TokenStream;
use quote::__private::Span;
use quote::quote;
use syn::{parse_macro_input, ItemTrait, TraitItem, Ident, parse_quote};

#[proc_macro_attribute]
pub fn seamock(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input trait
    let input = parse_macro_input!(input as ItemTrait);

    let mock_struct_name = Ident::new(&format!("Mock{}", &input.ident), Span::call_site());

    // Get the trait methods
    let times_trait_attrs = input.items.iter().filter_map(|item| {
        if let TraitItem::Method(method) = item {
            Some(
                Ident::new(&format!("times_{}", &method.sig.ident), method.sig.ident.span()),
            )
        } else {
            None
        }
    });

    let expect_trait_methods = input.items.iter().filter_map(|item| {
        if let TraitItem::Method(method) = item {
            Some(
                Ident::new(&format!("expect_times_{}", &method.sig.ident), method.sig.ident.span()),
            )
        } else {
            None
        }
    });

    let impl_methods = input.items.iter().filter_map(|item| {
        if let TraitItem::Method(method) = item {
            let method_name = Ident::new(&format!("{}_with", &method.sig.ident), method.sig.ident.span());
            let method_output = &method.sig.output;
            let ret_type: syn::Type = match &method.sig.output {
                syn::ReturnType::Type(_, x) => *x.to_owned(),
                syn::ReturnType::Default => parse_quote!{ () }
            };
            let method_inputs = &method.sig.inputs;
            let times_attr = Ident::new(&format!("times_{}", &method.sig.ident), method.sig.ident.span());
            Some (quote! {
                fn #method_name(#method_inputs, _r: #ret_type) #method_output {
                    self.#times_attr.replace_with(|&mut old| old + 1);
                    _r
                }
            })
        } else {
            None
        }
    });

    let x = times_trait_attrs.clone();

    // Generate the MockContext struct with RefCell fields for each method
    let mock_struct = quote! {
        struct #mock_struct_name {
            #(
                #x: std::cell::RefCell<u8>,
            )*
        }
    };

    let x = times_trait_attrs.clone();

    // Implement the trait for MockContext
    let mock_impl = quote! {
        impl #mock_struct_name {
            pub fn new() -> Self {
                Self {
                    #(
                        #times_trait_attrs: RefCell::new(0),
                    )*
                }
            }
            #(
                fn #expect_trait_methods(&self, times: u8) -> bool {
                    self.#x.borrow().clone() == times
                }
            )*
            #(#impl_methods)*
        }
    };

    let original = &input.ident;
    let trait_impl = quote! {
        impl #original for #mock_struct_name { }
    };

    // Combine the generated tokens
    let expanded = quote! {
        use core::cell::RefCell;
        #input
        #mock_struct
        #mock_impl
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