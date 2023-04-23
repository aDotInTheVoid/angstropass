use proc_macro2::TokenStream;
use quote::quote;

mod input;
mod lang;
mod lower;
mod parse;

/**
```text
┌─────────────────────────┐
│ proc_macro::TokenStream │
└─────────────────────────┘
  │
  │ syn::parse
  ▼
┌─────────────────────────┐
│      syn::ItemMod       │
└─────────────────────────┘
  │
  │ parse::parse
  ▼
┌─────────────────────────┐
│      input::Input       │
└─────────────────────────┘
  │
  │ lower
  ▼
┌─────────────────────────┐
│      langs::Langs       │
└─────────────────────────┘
  │
  │ todo
  ▼
┌─────────────────────────┐
│ proc_macro::TokenStream │
└─────────────────────────┘
```
 */

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

    let input = parse::parse(module)?;

    let mut lowerer = lower::Lowerer::default();

    for l in input.langs {
        lowerer.add_lang(l)?;
    }

    Ok(quote! {})
}

// impl Langs {
//     fn add_lang(&mut self, module: &syn::ItemMod) -> Result<(), syn::Error> {
//         let name = module.ident.to_string();
//         if self.has_lang(&name) {
//             return Err(syn::Error::new_spanned(
//                 &module.ident,
//                 format!("duplicate language: {}", name),
//             ));
//         }

//         let extends = self.extract_extends(module)?;

//         module_content(module)?;

//         self.langs.push(Lang { name, extends });

//         Ok(())
//     }

//     fn extract_extends(&mut self, module: &syn::ItemMod) -> Result<Option<String>, syn::Error> {
//         let extends = match &module.attrs[..] {
//             [] => None,
//             [attr] => {
//                 let attr = attr.meta.require_list()?;
//                 if attr.path.is_ident("extends") {
//                     let name: syn::Ident = attr.parse_args()?;
//                     let ns = name.to_string();
//                     if !self.has_lang(&ns) {
//                         return Err(syn::Error::new_spanned(
//                             &name,
//                             format!("unknown language: {}", ns),
//                         ));
//                     } else {
//                         Some(ns)
//                     }
//                 } else {
//                     let err = syn::Error::new_spanned(&attr.path, "expected #[extends]");
//                     return Err(err);
//                 }
//             }
//             [_, attr2, ..] => {
//                 let err =
//                     syn::Error::new_spanned(&attr2, "only one #[extends] attribute is allowed");
//                 return Err(err);
//             }
//         };
//         Ok(extends)
//     }

//     fn has_lang(&mut self, name: &str) -> bool {
//         self.langs.iter().any(|l| l.name == *name)
//     }
// }

// fn module_content(module: &syn::ItemMod) -> Result<&[syn::Item], syn::Error> {
//     match module.content {
//         Some((_, ref content)) => Ok(content),
//         None => Err(syn::Error::new_spanned(
//             module,
//             format!("expected an inline module"),
//         )),
//     }
// }
