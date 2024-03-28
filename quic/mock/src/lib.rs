extern crate proc_macro;

use proc_macro::TokenStream;
use quote::__private::Span;
use quote::quote;
use syn::{parse_macro_input, ItemTrait, TraitItem, Ident, parse_quote, TraitItemMethod};

fn generate_attr_names(method: &TraitItemMethod, prefixes: &[&str]) -> Vec<Ident> {
    prefixes
        .iter()
        .map(|prefix| Ident::new(&format!("{}_{}", prefix, &method.sig.ident), method.sig.ident.span()))
        .collect()
}

#[proc_macro_attribute]
pub fn seamock(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input trait
    let input = parse_macro_input!(input as ItemTrait);

    let mock_struct_name = Ident::new(&format!("Mock{}", &input.ident), Span::call_site());

    let trait_methods = input.items.iter().filter_map(|item| {
        if let TraitItem::Method(method) = item {
            Some(method)
        } else {
            None
        }
    });

    let max_times = trait_methods.clone().flat_map(|method| {
        generate_attr_names(method, &["max_times"])
    });

    let times = trait_methods.clone().flat_map(|method| {
        generate_attr_names(method, &["times"])
    });

    let mut returning_attrs = vec!{};
    let mut with_attrs = vec!{};
    let mut with_methods = vec!{};

    let ret = trait_methods.clone().flat_map(|method| {
        let method_output = &method.sig.output;
        let mut method_inputs = method.sig.inputs.clone();
        let mut val_with_params = vec![];
        // Check if the first argument is `&self` and remove it
        if let Some(first_arg) = method_inputs.first() {
            if let syn::FnArg::Receiver(_) = first_arg {
                method_inputs = method_inputs.iter().skip(1).cloned().collect();
            }
        }
        // For each argument, create WithVal<T> where T is the argument type
        for arg in method_inputs.iter() {
            if let syn::FnArg::Typed(pat_type) = arg {
                let arg_type = &pat_type.ty;
                let with_val_type: syn::Type = parse_quote! { WithVal<#arg_type> };
                val_with_params.push(with_val_type);
            }
        }
        // Create a tuple of WithVal<T> for each argument
        let val_with_tuple = if val_with_params.is_empty() {
            None
        } else {
            Some(quote! { ( #(#val_with_params),* ) })
        };
        let returning_attr = Ident::new(&format!("val_returning_{}", &method.sig.ident), method.sig.ident.span());
        let with_attr = Ident::new(&format!("val_with_{}", &method.sig.ident), method.sig.ident.span());
        let with_method = Ident::new(&format!("with_{}", &method.sig.ident), method.sig.ident.span());
        returning_attrs.push(quote! {
            #returning_attr: |#method_inputs| Default::default(),
        });
        if val_with_tuple.is_some() {
            with_attrs.push(quote! {
                #with_attr: None,
            });
            with_methods.push(quote! {
                fn #with_method(&mut self, with: Option<#val_with_tuple>) -> &mut Self {
                    self.#with_attr = Some(with);
                    self
                }
            });
            quote! {
                #returning_attr: fn(#method_inputs) #method_output,
                #with_attr: Option<#val_with_tuple>,
            }
        } else {
            quote! {
                #returning_attr: fn(#method_inputs) #method_output,
            }
        }
    });

    let ret_methods = trait_methods.clone().flat_map(|method| {
        let method_output = &method.sig.output;
        let mut method_inputs = method.sig.inputs.clone();
        // Remove `&self`
        if let Some(first_arg) = method_inputs.first() {
            if let syn::FnArg::Receiver(_) = first_arg {
                method_inputs = method_inputs.iter().skip(1).cloned().collect();
            }
        }
        let input_method = Ident::new(&format!("returning_{}", &method.sig.ident), method.sig.ident.span());
        let update_attr = Ident::new(&format!("val_returning_{}", &method.sig.ident), method.sig.ident.span());
        Some (quote! {
            fn #input_method(&mut self, f: fn(#method_inputs) #method_output) -> &mut Self {
                self.#update_attr = f;
                self
            }
        })
    });


    let times_clone = times.clone();
    let max_times_clone = max_times.clone();

    // Generate the Mock struct
    let mock_struct = quote! {
        struct #mock_struct_name {
            #(
                #max_times_clone: u64,
            )*
            #(
                #times_clone: std::cell::RefCell<u64>,
            )*
            #(
                #ret
            )*
        }
    };

    let times_clone = times.clone();
    let max_times_clone = max_times.clone();

    // Implement the trait for MockContext
    let mock_impl = quote! {
        impl #mock_struct_name {
            pub fn new() -> Self {
                Self {
                    #(
                        #max_times_clone: u64::MAX,
                    )*
                    #(
                        #times_clone: RefCell::new(0),
                    )*
                    #(
                        #returning_attrs
                    )*
                    #(
                        #with_attrs
                    )*
                }
            }
            #(#ret_methods)*
            #(#with_methods)*
            // #(
            //     fn #expect_trait_methods(&self, times: u64) -> bool {
            //         *self.#x.borrow() == times
            //     }
            // )*
            // #(#impl_methods)*
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

