use syn::{
    punctuated::Iter, Data, DeriveInput, Field, Fields, GenericArgument, Path, PathArguments, Type,
};

pub fn is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(x) => {
            if let Some(x) = get_first_ident(&x.path) {
                return x == "Option";
            } else {
                return false;
            }
        }
        _ => return false,
    }
}

pub fn is_vec(ty: &Type) -> bool {
    match ty {
        Type::Path(x) => {
            if let Some(x) = get_first_ident(&x.path) {
                return x == "Vec";
            } else {
                return false;
            }
        }
        _ => return false,
    }
}

pub fn get_first_ident(path: &Path) -> Option<String> {
    if let Some(ref x) = path.segments.iter().nth(0) {
        return Some(x.ident.to_string());
    }
    None
}

pub fn get_inside(path: &Type) -> Option<Type> {
    match path {
        Type::Path(ref x) => {
            if x.path.segments.iter().len() != 1 {
                return None;
            }
            let x = x.path.segments.iter().nth(0).unwrap();
            match x.arguments {
                PathArguments::AngleBracketed(ref x) => {
                    if x.args.iter().len() != 1 {
                        return None;
                    }
                    match x.args.iter().nth(0).unwrap() {
                        GenericArgument::Type(ref x) => Some(x.clone()),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

pub trait FieldsIter<'a, T: 'a> {
    type Iterator: Iterator<Item = &'a T>;

    fn fields_iter(&'a self) -> Self::Iterator;
}

impl<'a> FieldsIter<'a, Field> for DeriveInput {
    type Iterator = Iter<'a, Field>;

    fn fields_iter(&'a self) -> Self::Iterator {
        match self.data {
            Data::Struct(ref x) => match x.fields {
                Fields::Named(ref x) => x.named.iter(),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}
