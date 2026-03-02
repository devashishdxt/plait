use syn::{Expr, Ident, LitBool, LitChar, LitFloat, LitInt, LitStr};

pub enum AttributeValue {
    LitStr(LitStr),
    LitChar(LitChar),
    LitInt(LitInt),
    LitFloat(LitFloat),
    LitBool(LitBool),
    Escaped(Expr),
    Raw(Expr),
}

pub struct NameValueAttribute {
    pub name: LitStr,
    pub is_maybe: bool,
    pub value: Option<AttributeValue>,
}

pub enum Attribute {
    Spread(Ident),
    NameValue(NameValueAttribute),
}
