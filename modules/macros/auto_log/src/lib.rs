use proc_macro::TokenStream;

use syn::{ItemFn, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn auto_log(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(input as ItemFn);

    let original_body = function.block.to_owned();
    function.block = parse_quote! {
        {
            let result = (|| async { #original_body })().await;
            log::info!("Response: {}", serde_json::to_string(&result).unwrap());
            result
        }
    };

    TokenStream::from(quote::quote!(#function))
}
