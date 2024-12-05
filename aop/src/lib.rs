use proc_macro::TokenStream;

mod aspect;

#[proc_macro_attribute]
pub fn basic_information(args: TokenStream, func: TokenStream) -> TokenStream {
    aspect::basic_information(args, func)
}
