extern crate proc_macro;

use proc_macro::TokenStream;

// Derive macros 过程宏
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(_input: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

// Attribute macros 属性宏
#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("result: {:?}", attr);
    println!("item: \"{}\"", item.to_string());
    item
}