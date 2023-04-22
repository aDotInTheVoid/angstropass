use proc_macro2::TokenStream;
// use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn langs(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let module = syn::parse_macro_input!(input as syn::ItemMod);
    langs_impl(args.into(), module).into()
}

fn langs_impl(args: TokenStream, module: syn::ItemMod) -> TokenStream {
    if !args.is_empty() {
        let err = syn::Error::new_spanned(args, "this attribute does not take any arguments");
        return err.into_compile_error().into();
    }

    let name = module.ident;

    quote! {
        mod #name {

        }
    }
}
