extern crate proc_macro;

use proc_macro::TokenStream;

use darling::{Error, FromMeta};
use darling::ast::NestedMeta;
use quote::ToTokens;
use syn::{parse_quote, Stmt};

#[derive(Debug, FromMeta)]
struct AuthorizeArgs {
    /// The permissions name.
    permission: String,

    claims_name: String,
}

#[proc_macro_attribute]
pub fn authorize(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => { v }
        Err(e) => { return TokenStream::from(Error::from(e).write_errors()); }
    };

    let args = match AuthorizeArgs::from_list(&args) {
        Ok(v) => { v }
        Err(e) => { return TokenStream::from(Error::from(e).write_errors()); }
    };

    let claims_name = args.claims_name;
    let permission = args.permission;

    // Parse the function signature
    let mut ast: syn::ItemFn = syn::parse(input).expect("Failed to parse function");

    ast.block.stmts.insert(0, parse_quote! {
        let is_permitted: bool = claims.validate_permissions(
            &HashSet::from(
                [
                    #permission.to_string()
                ]
            )
        );
    });
    ast.block.stmts.insert(1, parse_quote! {
        if !is_permitted {
            return Err(HttpResponse::Forbidden().json(ErrorMessage {
                error: Some("insufficient_permissions".to_string()),
                error_description: Some(format!("Requires {}", #permission)),
                message: "Permission denied".to_string(),
            }));
        }
    });

    ast.into_token_stream().into()
}
