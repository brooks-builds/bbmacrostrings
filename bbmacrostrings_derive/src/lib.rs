extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ToString)]
pub fn to_string_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_to_string_macro(&ast)
}

fn impl_to_string_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    let mut arms = vec![];
    if let syn::Data::Enum(data_enum) = data {
        for variant in &data_enum.variants {
            arms.push(&variant.ident);
        }
    } else {
        panic!("we are not a data enum, why you do this?");
    }
    let gen = quote! {
        impl ToString for #name {
            fn to_string(&self) -> String {
                match self {
                    #(#name::#arms => stringify!(#arms).to_string(),)*
                }
            }
            fn to_str(&self) -> &'static str {
                match self {
                    #(#name::#arms => stringify!(#arms),)*
                }
            }
        }
    };
    gen.into()
}
