use proc_macro2::Span;
use syn::{parse::Parse, punctuated::Punctuated, Error, Expr, Token};

#[derive(Default, Debug)]
pub struct AssignAttrs;

impl Parse for AssignAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;

        let assign_exprs = attrs.into_iter().filter_map(|attr| {
            if let Expr::Assign(assign_expr) = attr {
                if let Expr::Path(path) = *assign_expr.left {
                    return Some((path, assign_expr.right));
                }
            }

            None
        });

        for (assign_expr, _right_expr) in assign_exprs {
            let name = assign_expr
                .path
                .get_ident()
                .ok_or_else(|| {
                    Error::new_spanned(assign_expr.clone(), "config attribute must have a name")
                })?
                .to_string();

            if name == "default"
                || name == "serialize_with"
                || name == "deserialize_with"
                || name == "with"
            {
                return Ok(Self {});
            }
        }

        Err(Error::new(Span::call_site(), "not found"))
    }
}
