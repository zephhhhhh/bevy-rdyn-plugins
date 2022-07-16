use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use rdyn_plugins::CREATE_RDYN_SYM_NAME;

/// Macro derive for structs implementing the bevy Plugin trait
/// that marks the plugin as the main or "entry" plugin for the dynamic plugin.
#[proc_macro_derive(RDynPlugin)]
pub fn rdyn_plugin_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    let func_name = syn::Ident::new(
        std::str::from_utf8(CREATE_RDYN_SYM_NAME).unwrap(),
        struct_name.span(),
    );

    TokenStream::from(quote! {
        #[no_mangle]
        pub extern "Rust" fn #func_name() -> RDynReturn {
            Box::new(#struct_name {})
        }
    })
}
