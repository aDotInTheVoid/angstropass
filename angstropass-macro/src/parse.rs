use crate::input;

// Don't preform any semantic validation here.

pub(crate) fn parse(input: syn::ItemMod) -> Result<input::Input, syn::Error> {
    let docs = docs(&input.attrs);
    let name = input.ident;

    let langs = vec![];

    Ok(input::Input { docs, name, langs })
}

fn docs(attrs: &[syn::Attribute]) -> Vec<syn::Attribute> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .cloned()
        .collect()
}
