use syn::*;
use ext::*;
use quote::Tokens;
use spanned::Spanned;

use FieldMember;

pub trait CodegenFieldsExt {
    fn surround(&self, tokens: Tokens) -> Tokens;
    fn ignore_tokens(&self) -> Tokens;
    fn id_match_tokens(&self) -> Tokens;
}

pub fn field_to_ident(i: usize, field: &Field) -> Ident {
    let name = match field.ident {
        Some(id) => format!("_{}", id),
        None => format!("_{}", i)
    };

    Ident::new(&name, field.span().into())
}

pub fn field_to_match((i, field): (usize, &Field)) -> Tokens {
    let ident = field_to_ident(i, field);
    match field.ident {
        Some(id) => quote!(#id: #ident),
        None => quote!(#ident)
    }
}

impl CodegenFieldsExt for Fields {
    fn surround(&self, tokens: Tokens) -> Tokens {
        match *self {
            Fields::Named(..) => quote!({ #tokens }),
            Fields::Unnamed(..) => quote!(( #tokens )),
            Fields::Unit => quote!()
        }
    }

    fn ignore_tokens(&self) -> Tokens {
        self.surround(quote!(..))
    }

    fn id_match_tokens(&self) -> Tokens {
        let idents = self.iter()
            .enumerate()
            .map(field_to_match);

        self.surround(quote!(#(#idents),*))
    }
}

pub trait TokensExt {
    fn tokens(&self) -> Tokens;
}

impl<'f> TokensExt for FieldMember<'f> {
    fn tokens(&self) -> Tokens {
        let index = self.member.unnamed().map(|i| i.index).unwrap_or(0);
        let ident = field_to_ident(index as usize, &self.field);
        quote!(#ident)
    }
}

// use rocket::http::{ContentType, MediaType, Status};

// impl TokensExt for ContentType {
//     fn tokens(&self) -> Tokens {
//         let mt_tokens = self.0.tokens();
//         quote!(rocket::http::ContentType(#mt_tokens))
//     }
// }

// impl TokensExt for MediaType {
//     fn tokens(&self) -> Tokens {
//         let (top, sub) = (self.top().as_str(), self.sub().as_str());
//         let (keys, values) = (self.params().map(|(k, _)| k), self.params().map(|(_, v)| v));
//         quote!(rocket::http::MediaType {
//             source: rocket::http::Source::None,
//             top: rocket::http::IndexedStr::Concrete(
//                 std::borrow::Cow::Borrowed(#top)
//             ),
//             sub: rocket::http::IndexedStr::Concrete(
//                 std::borrow::Cow::Borrowed(#sub)
//             ),
//             params: rocket::http::MediaParams::Static(&[
//                 #((
//                     rocket::http::IndexedStr::Concrete(std::borrow::Cow::Borrowed(#keys)),
//                     rocket::http::IndexedStr::Concrete(std::borrow::Cow::Borrowed(#values))
//                 )),*
//             ])
//         })
//     }
// }

// impl TokensExt for Status {
//     fn tokens(&self) -> Tokens {
//         let (code, reason) = (self.code, self.reason);
//         quote!(rocket::http::Status { code: #code, reason: #reason })
//     }
// }
