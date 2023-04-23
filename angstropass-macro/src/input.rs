pub struct Input {
    pub docs: Vec<syn::Attribute>,

    pub name: syn::Ident,

    pub langs: Vec<Lang>,
}

pub struct Lang {
    pub name: syn::Ident,
    pub docs: Vec<syn::Attribute>,
    pub extends: Option<syn::Ident>,
}
