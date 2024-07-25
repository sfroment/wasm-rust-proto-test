extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(Accessors)]
pub fn accessors_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = match input.data {
        Data::Struct(data) => {
            let mut methods = vec![];
            let mut constructor_params = vec![];
            let mut constructor_assignments = vec![];

            if let Fields::Named(fields) = data.fields {
                for field in fields.named {
                    let field_name = field.ident.clone().unwrap();
                    let ty = field.ty.clone();
                    let setter_name = Ident::new(&format!("set_{}", field_name), field_name.span());

                    // Constructor parameters and assignments
                    constructor_params.push(quote! { #field_name: #ty });
                    constructor_assignments.push(quote! { #field_name });

                    methods.push(quote! {
                        // Getter
                        #[wasm_bindgen(getter)]
                        pub fn #field_name(&self) -> #ty {
                            self.#field_name.clone()
                        }

                        // Setter
                        #[wasm_bindgen(setter)]
                        pub fn #setter_name(&mut self, value: #ty) {
                            self.#field_name = value;
                        }
                    });
                }
            }

            // Generate the constructor function
            let constructor = quote! {
                #[wasm_bindgen(constructor)]
                pub fn new(#(#constructor_params),*) -> Self {
                    Self {
                        #(#constructor_assignments),*
                    }
                }
            };

            quote! {
                #[wasm_bindgen]
                impl #name {
                    #constructor
                    #(#methods)*
                }
            }
        }
        _ => panic!("Accessors derive macro can only be used with structs"),
    };

    TokenStream::from(expanded)
}
