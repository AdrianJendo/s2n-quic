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
            let method_name = &method.sig.ident;
            let method_output = &method.sig.output;
            let ret_type: syn::Type = match &method.sig.output {
                syn::ReturnType::Type(_, x) => *x.to_owned(),
                syn::ReturnType::Default => parse_quote!{ () }
            };
            let method_inputs = &method.sig.inputs;
            let times_attr = Ident::new(&format!("times_{}", &method.sig.ident), method.sig.ident.span());
            Some (quote! {
                fn #method_name(#method_inputs, r: #ret_type) #method_output {
                    self.#times_attr.replace_with(|&mut old| old + 1);
                    r
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
                #x: std::cell::RefCell<u64>,
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
                fn #expect_trait_methods(&self, times: u64) -> bool {
                    *self.#x.borrow() == times
                }
            )*
            #(#impl_methods)*
        }
    };

    // let original = &input.ident;
    // let trait_impl = quote! {
    //     impl #original for #mock_struct_name { }
    // };

    // Combine the generated tokens
    let expanded = quote! {
        use core::cell::RefCell;
        #input
        #mock_struct
        #mock_impl
        // #trait_impl
    };


    TokenStream::from(expanded)
}

