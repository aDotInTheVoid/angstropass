use std::collections::{btree_map, BTreeMap};

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
    let lang_docs = docs(&module.attrs);

    let name = module.ident.clone();

    let extends = get_extends(&module.attrs)?;

    let mut types = BTreeMap::new();

    for item in module_content(module)? {
        match item {
            syn::Item::Type(type_alias) => match types.entry(type_alias.ident.clone()) {
                btree_map::Entry::Vacant(e) => {
                    let info = input::TypeAlias {
                        docs: docs(&type_alias.attrs),
                        kind: kind(&type_alias.attrs)?,
                        ty: *type_alias.ty.clone(),
                    };

                    e.insert(info);
                }
                btree_map::Entry::Occupied(old) => {
                    return Err(syn::Error::new_spanned(
                        type_alias,
                        format!("duplicate definition of {}", old.key()),
                    ))
                }
            },

            syn::Item::Struct(_) => {}
            syn::Item::Enum(_) => {}

            _ => return Err(syn::Error::new_spanned(item, format!("expected a type"))),
        }
    }

    Ok(input::Lang {
        docs: lang_docs,
        name,
        extends,
        types,
    })
}

fn kind(attrs: &[syn::Attribute]) -> Result<Option<input::DeltaKind>, syn::Error> {
    let mut kind = None;

    for attr in attrs {
        let attrs_kind = if attr.path().is_ident("add") {
            Some(input::DeltaKind::Add)
        } else if attr.path().is_ident("replace") {
            Some(input::DeltaKind::Replace)
        } else if attr.path().is_ident("delete") {
            Some(input::DeltaKind::Delete)
        } else {
            None
        };

        if let Some(attr_kind) = attrs_kind {
            if kind.is_some() {
                return Err(syn::Error::new_spanned(
                    attr,
                    "only one of #[add], #[replace], or #[delete] is allowed",
                ));
            }

            kind = Some(attr_kind);
        }
    }

    Ok(kind)
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
