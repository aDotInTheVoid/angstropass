use proc_macro2::TokenStream;
// use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn langs(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let module = syn::parse_macro_input!(input as syn::ItemMod);
    match langs_impl(args.into(), module) {
        Ok(toks) => toks.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn langs_impl(args: TokenStream, module: syn::ItemMod) -> Result<TokenStream, syn::Error> {
    if !args.is_empty() {
        let err = syn::Error::new_spanned(args, "this attribute does not take any arguments");
        return Err(err);
    }

    let content = module_content(&module)?;

    let mut comp = Compiler::default();

    for item in content {
        if let syn::Item::Mod(module) = item {
            comp.add_lang(&module)?;
        } else {
            let err = syn::Error::new_spanned(item, "only modules are allowed in #[langs]");
            return Err(err);
        }
    }

    Ok(quote! {})
}

#[derive(Default)]
struct Compiler {
    langs: Vec<String>,
}

impl Compiler {
    fn add_lang(&mut self, module: &syn::ItemMod) -> Result<(), syn::Error> {
        let name = module.ident.to_string();
        if self.langs.contains(&name) {
            return Err(syn::Error::new_spanned(
                &module.ident,
                format!("duplicate language: {}", name),
            ));
        }

        module_content(module)?;

        self.langs.push(name);

        Ok(())
    }
}

fn module_content(module: &syn::ItemMod) -> Result<&[syn::Item], syn::Error> {
    match module.content {
        Some((_, ref content)) => Ok(content),
        None => Err(syn::Error::new_spanned(
            module,
            format!("expected an inline module"),
        )),
    }
}
