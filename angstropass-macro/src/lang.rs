#[derive(Debug)]
pub struct Lang {
    pub name: syn::Ident,
    pub docs: Vec<syn::Attribute>,
    /// Fully elabourated types
    pub nodes: Nodes,

    pub kind: LangKind,
}

#[derive(Debug)]
pub struct Nodes {
    // TODO:
}

#[derive(Debug)]
pub enum LangKind {
    /// Doesn't Inherit
    Base,

    Derivitive {
        from: syn::Ident,
    },
}
