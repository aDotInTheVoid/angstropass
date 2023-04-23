use std::collections::BTreeMap;

pub struct Input {
    pub docs: Vec<syn::Attribute>,

    pub name: syn::Ident,

    pub langs: Vec<Lang>,
}

pub struct Lang {
    pub name: syn::Ident,
    pub docs: Vec<syn::Attribute>,
    pub extends: Option<syn::Ident>,

    pub types: BTreeMap<syn::Ident, TypeAlias>,
}

pub enum DeltaKind {
    /// Change a definition.
    Replace,

    /// Delete a definition.
    Delete,

    /// Add a definition.
    Add,
}

pub struct TypeAlias {
    pub docs: Vec<syn::Attribute>,
    pub kind: Option<DeltaKind>,
    pub ty: syn::Type,
}
