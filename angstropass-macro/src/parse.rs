use crate::input;

// Don't preform any semantic validation here.

pub(crate) fn parse(input: syn::ItemMod) -> Result<input::Input, syn::Error> {
    let docs = docs(&input.attrs);

    let lang_mods = module_content(&input)?
        .iter()
        .map(|item| match item {
            syn::Item::Mod(module) => Ok(module),
            _ => Err(syn::Error::new_spanned(item, format!("expected a module"))),
        })
        .collect::<Result<Vec<_>, _>>()?;

    let langs = lang_mods
        .into_iter()
        .map(parse_lang)
        .collect::<Result<Vec<_>, _>>()?;

    let name = input.ident;

    Ok(input::Input { docs, name, langs })
}

fn parse_lang(module: &syn::ItemMod) -> Result<input::Lang, syn::Error> {
    let docs = docs(&module.attrs);

    let name = module.ident.clone();

    let extends = get_extends(&module.attrs)?;

    Ok(input::Lang {
        docs,
        name,
        extends,
    })
}

fn docs(attrs: &[syn::Attribute]) -> Vec<syn::Attribute> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .cloned()
        .collect()
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

/// Extract `foo` from `#[extends(foo)]`.
fn get_extends(attrs: &[syn::Attribute]) -> Result<Option<syn::Ident>, syn::Error> {
    let mut extends = None;

    for attr in attrs {
        if attr.path().is_ident("extends") {
            let name: syn::Ident = attr.parse_args()?;
            if extends.is_some() {
                return Err(syn::Error::new_spanned(
                    attr,
                    format!("only one #[extends] attribute is allowed"),
                ));
            } else {
                extends = Some(name);
            }
        }
    }

    Ok(extends)
}
