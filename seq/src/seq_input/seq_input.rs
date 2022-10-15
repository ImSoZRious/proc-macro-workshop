use core::fmt::Debug;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{braced, parse::Parse, Ident, LitInt, Result, Token};

use super::helper;

pub struct SeqInput {
    name: Ident,
    start: LitInt,
    end: LitInt,
    stmts: TokenStream,
    has_section: bool,
    inclusive: bool,
}

impl SeqInput {
    pub fn body(&self) -> Result<TokenStream> {
        let start: usize = match self.start.base10_parse() {
            Ok(n) => n,
            Err(_) => panic!(),
        };

        let end: usize = match self.end.base10_parse() {
            Ok(n) => n,
            Err(_) => panic!(),
        };

        if self.has_section {
            return helper::expand_sections(
                self.stmts.clone(),
                &self.name,
                start,
                end,
                self.inclusive,
            );
        } else {
            return helper::expand_all(self.stmts.clone(), &self.name, start, end, self.inclusive);
        }
    }
}

impl ToTokens for SeqInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.body());
    }
}

impl Debug for SeqInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body = self.body().unwrap();
        writeln!(
            f,
            "name: {}, start: {}, end: {}, has_section: {}, body: \n{}",
            self.name,
            self.start.to_string(),
            self.end.to_string(),
            match self.has_section {
                true => "YES",
                false => "NO",
            },
            body,
        )?;

        Ok(())
    }
}

impl Parse for SeqInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let start: LitInt = input.parse()?;
        let inclusive: bool;
        if input.peek(Token![..=]) {
            inclusive = true;
            input.parse::<Token![..=]>()?;
        } else {
            inclusive = false;
            input.parse::<Token![..]>()?;
        }
        let end: LitInt = input.parse()?;
        let content;
        let _ = braced!(content in input);

        let stmts = TokenStream::parse(&content)?;

        let has_section = helper::has_section(&stmts);

        Ok(Self {
            name,
            start,
            end,
            stmts,
            has_section,
            inclusive,
        })
    }
}
