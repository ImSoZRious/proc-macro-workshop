use super::utils;
use proc_macro2::{Delimiter, Group, Literal, TokenStream, TokenTree};
use quote::TokenStreamExt;
use syn::{Ident, Result};

fn expand(ts: TokenStream, ident: &Ident, n: usize) -> Result<TokenStream> {
    let mut ret = TokenStream::new();
    let int = Literal::usize_unsuffixed(n);

    let mut iter = ts.into_iter();

    utils::init!(a, b, c; iter.next());

    while a.is_some() {
        match (&a, &b, &c) {
            (
                Some(TokenTree::Ident(ref x)),
                Some(TokenTree::Punct(ref y)),
                Some(TokenTree::Ident(ref z)),
            ) if y.as_char() == '~' && z == ident => {
                let tt = TokenTree::Ident(Ident::new(&format!("{}{}", x, int), x.span()));

                ret.append(tt);
                utils::clear!(a, b, c; iter.next());
            }

            (Some(TokenTree::Ident(ref x)), _, _) if x == ident => {
                ret.append(TokenTree::Literal(int.clone()));
                utils::shift!(a, b, c; iter.next());
            }

            (Some(TokenTree::Group(ref x)), _, _) => {
                let ts = expand(x.stream(), ident, n)?;
                let mut tt = Group::new(x.delimiter(), ts);

                tt.set_span(x.span());

                ret.append(tt);
                utils::shift!(a, b, c; iter.next());
            }

            (_, _, _) => {
                ret.append(a.unwrap().clone());
                utils::shift!(a, b, c; iter.next());
            }
        }
    }

    Ok(ret)
}

pub fn expand_all(
    ts: TokenStream,
    ident: &Ident,
    start: usize,
    end: usize,
    inclusive: bool,
) -> Result<TokenStream> {
    if inclusive {
        return (start..=end)
            .map(|i| expand(ts.clone(), ident, i))
            .collect();
    } else {
        return (start..end).map(|i| expand(ts.clone(), ident, i)).collect();
    }
}

pub fn expand_sections(
    ts: TokenStream,
    ident: &Ident,
    start: usize,
    end: usize,
    inclusive: bool,
) -> Result<TokenStream> {
    let mut tmp_iter = ts.clone().into_iter();

    utils::init!(a, b, c; tmp_iter.next());

    let mut ret = TokenStream::new();

    while a.is_some() {
        match (&a, &b, &c) {
            (
                Some(TokenTree::Punct(ref x)),
                Some(TokenTree::Group(ref y)),
                Some(TokenTree::Punct(ref z)),
            ) if x.as_char() == '#'
                && y.delimiter() == Delimiter::Parenthesis
                && z.as_char() == '*' =>
            {
                let expanded = if inclusive {
                    (start..=end)
                        .map(|i| expand(y.stream(), ident, i))
                        .collect::<Result<TokenStream>>()?
                } else {
                    (start..end)
                        .map(|i| expand(y.stream(), ident, i))
                        .collect::<Result<TokenStream>>()?
                };
                ret.extend(expanded);

                utils::clear!(a, b, c; tmp_iter.next());
            }

            (Some(TokenTree::Group(ref x)), _, _) => {
                let mut expanded = Group::new(
                    x.delimiter(),
                    expand_sections(x.stream(), ident, start, end, inclusive)?,
                );
                expanded.set_span(x.span());
                ret.append(expanded);

                utils::shift!(a, b, c; tmp_iter.next());
            }

            (_, _, _) => {
                ret.append(a.unwrap());

                utils::shift!(a, b, c; tmp_iter.next());
            }
        }
    }

    Ok(ret)
}

pub fn has_section(input: &TokenStream) -> bool {
    let mut iter = input.clone().into_iter();
    let mut ret = false;

    utils::init!(a, b, c; iter.next());

    while a.is_some() && !ret {
        match (&a, &b, &c) {
            (
                Some(TokenTree::Punct(ref x)),
                Some(TokenTree::Group(ref y)),
                Some(TokenTree::Punct(ref z)),
            ) if x.as_char() == '#'
                && y.delimiter() == Delimiter::Parenthesis
                && z.as_char() == '*' =>
            {
                ret = true;
            }

            (Some(TokenTree::Group(ref x)), _, _) => {
                ret = has_section(&x.stream());

                utils::shift!(a, b, c; iter.next());
            }

            (_, _, _) => {
                utils::shift!(a, b, c; iter.next());
            }
        }
    }

    ret
}
