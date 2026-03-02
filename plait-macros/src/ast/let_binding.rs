use syn::{Expr, Pat};

pub struct LetBinding {
    pub pattern: Pat,
    pub expr: Option<Expr>,
}
