use syn::{Expr, Ident, LitBool, LitChar, LitFloat, LitInt, LitStr};

use crate::ast::{ComponentCall, Element, ForLoop, IfCondition, LetBinding, MatchExpression};

pub enum Node {
    Doctype,
    LitStr(LitStr),
    LitChar(LitChar),
    LitInt(LitInt),
    LitFloat(LitFloat),
    LitBool(LitBool),
    Escaped(Expr),
    Raw(Expr),
    LetBinding(LetBinding),
    IfCondition(IfCondition),
    MatchExpression(MatchExpression),
    ForLoop(ForLoop),
    Element(Element),
    Block(Vec<Node>),
    Children(Ident),
    ComponentCall(ComponentCall),
}
