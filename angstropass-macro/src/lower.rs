use crate::{
    input,
    lang::{self, Nodes},
};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Lowerer {
    langs: HashMap<String, lang::Lang>,
}

impl Lowerer {
    pub fn add_lang(&mut self, lang: input::Lang) -> Result<(), syn::Error> {
        let name = lang.name.to_string();

        if self.has_lang(&lang.name) {
            let err =
                syn::Error::new_spanned(&lang.name, format!("duplicate lang `{}`", lang.name));
            return Err(err);
        }

        if let Some(extends) = &lang.extends {
            if !self.has_lang(extends) {
                let err = syn::Error::new_spanned(
                    extends,
                    format!("lang `{}` extends unknown lang `{}`", lang.name, extends),
                );
                return Err(err);
            }
        }

        let nodes = Nodes {};
        let kind = match lang.extends {
            None => lang::LangKind::Base,
            Some(extends) => lang::LangKind::Derivitive { from: extends },
        };

        let newlang = lang::Lang {
            name: lang.name,
            docs: lang.docs,
            nodes,
            kind,
        };

        self.langs.insert(name, newlang);

        Ok(())
    }

    fn has_lang(&self, name: &syn::Ident) -> bool {
        self.langs.contains_key(&name.to_string())
    }
}
