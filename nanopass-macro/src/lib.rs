use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn langs(args: TokenStream, input: TokenStream) -> TokenStream {
    let module = syn::parse_macro_input!(input as syn::ItemMod);

    let name = module.ident;

    quote! {
        mod #name {

        }
    }
    .into()
}
