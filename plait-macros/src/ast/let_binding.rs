use syn::{
    Expr, Pat, PatType, Type,
    parse::{Parse, ParseStream},
    token::{Colon, Eq, Let, Semi},
};

pub struct LetBinding {
    pub pattern: Pat,
    pub expr: Option<Expr>,
}

impl Parse for LetBinding {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let _: Let = input.parse()?;
        let mut pattern = Pat::parse_single(input)?;

        if input.peek(Colon) {
            let colon_token: Colon = input.parse()?;
            let ty: Type = input.parse()?;
            pattern = Pat::Type(PatType {
                attrs: Vec::new(),
                pat: Box::new(pattern),
                colon_token,
                ty: Box::new(ty),
            });
        }

        let expr = if input.peek(Eq) {
            let _: Eq = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        let _: Semi = input.parse()?;

        Ok(Self { pattern, expr })
    }
}
