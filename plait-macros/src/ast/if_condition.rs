use syn::{
    Expr, braced,
    parse::{Parse, ParseStream},
    token::{At, Else, If},
};

use crate::ast::Node;

#[derive(Debug)]
pub struct IfCondition {
    pub condition: Expr,
    pub then_branch: Vec<Node>,
    pub else_branch: Option<ElseBranch>,
}

impl Parse for IfCondition {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        parse_if_condition(input)
    }
}

#[derive(Debug)]
pub enum ElseBranch {
    If(Box<IfCondition>),
    Else(Vec<Node>),
}

impl Parse for ElseBranch {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        parse_else_branch(input)
    }
}

fn parse_if_condition(input: ParseStream<'_>) -> syn::Result<IfCondition> {
    let _: If = input.parse()?;
    // Use parse_without_eager_brace to avoid parsing `condition {}` as a struct literal
    let condition = input.call(Expr::parse_without_eager_brace)?;

    let content;
    let _ = braced!(content in input);

    let mut then_branch = Vec::new();

    while !content.is_empty() {
        then_branch.push(content.parse()?);
    }

    let else_branch = if input.peek(At) && input.peek2(Else) {
        let _: At = input.parse()?;
        let _: Else = input.parse()?;
        Some(input.parse()?)
    } else {
        None
    };

    Ok(IfCondition {
        condition,
        then_branch,
        else_branch,
    })
}

fn parse_else_branch(input: ParseStream<'_>) -> syn::Result<ElseBranch> {
    if input.peek(If) {
        Ok(ElseBranch::If(Box::new(input.parse()?)))
    } else {
        let content;
        let _ = braced!(content in input);

        let mut else_branch = Vec::new();

        while !content.is_empty() {
            else_branch.push(content.parse()?);
        }

        Ok(ElseBranch::Else(else_branch))
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    fn parse_if(tokens: proc_macro2::TokenStream) -> syn::Result<IfCondition> {
        syn::parse2(tokens)
    }

    #[test]
    fn test_simple_if_with_empty_body() {
        let input = quote! { if true {} };
        let result = parse_if(input).unwrap();

        assert!(result.then_branch.is_empty());
        assert!(result.else_branch.is_none());
    }

    #[test]
    fn test_simple_if_with_text_body() {
        let input = quote! { if show { "Hello" } };
        let result = parse_if(input).unwrap();

        assert_eq!(result.then_branch.len(), 1);
        assert!(matches!(result.then_branch[0], Node::Text(_)));
        assert!(result.else_branch.is_none());
    }

    #[test]
    fn test_if_with_multiple_nodes() {
        let input = quote! { if show { "Hello" "World" } };
        let result = parse_if(input).unwrap();

        assert_eq!(result.then_branch.len(), 2);
        assert!(matches!(result.then_branch[0], Node::Text(_)));
        assert!(matches!(result.then_branch[1], Node::Text(_)));
    }

    #[test]
    fn test_if_else_with_text() {
        let input = quote! { if show { "Yes" } @else { "No" } };
        let result = parse_if(input).unwrap();

        assert_eq!(result.then_branch.len(), 1);
        assert!(matches!(result.then_branch[0], Node::Text(_)));

        match &result.else_branch {
            Some(ElseBranch::Else(nodes)) => {
                assert_eq!(nodes.len(), 1);
                assert!(matches!(nodes[0], Node::Text(_)));
            }
            _ => panic!("expected ElseBranch::Else"),
        }
    }

    #[test]
    fn test_if_else_with_empty_branches() {
        let input = quote! { if condition {} @else {} };
        let result = parse_if(input).unwrap();

        assert!(result.then_branch.is_empty());
        match &result.else_branch {
            Some(ElseBranch::Else(nodes)) => assert!(nodes.is_empty()),
            _ => panic!("expected ElseBranch::Else"),
        }
    }

    #[test]
    fn test_if_else_if() {
        let input = quote! { if a { "A" } @else if b { "B" } };
        let result = parse_if(input).unwrap();

        assert_eq!(result.then_branch.len(), 1);

        match &result.else_branch {
            Some(ElseBranch::If(else_if)) => {
                assert_eq!(else_if.then_branch.len(), 1);
                assert!(else_if.else_branch.is_none());
            }
            _ => panic!("expected ElseBranch::If"),
        }
    }

    #[test]
    fn test_if_else_if_else() {
        let input = quote! { if a { "A" } @else if b { "B" } @else { "C" } };
        let result = parse_if(input).unwrap();

        assert_eq!(result.then_branch.len(), 1);

        match &result.else_branch {
            Some(ElseBranch::If(else_if)) => {
                assert_eq!(else_if.then_branch.len(), 1);
                match &else_if.else_branch {
                    Some(ElseBranch::Else(nodes)) => {
                        assert_eq!(nodes.len(), 1);
                    }
                    _ => panic!("expected ElseBranch::Else in else-if"),
                }
            }
            _ => panic!("expected ElseBranch::If"),
        }
    }

    #[test]
    fn test_multiple_else_if_chain() {
        let input = quote! { if a { "A" } @else if b { "B" } @else if c { "C" } @else { "D" } };
        let result = parse_if(input).unwrap();

        // First if
        match &result.else_branch {
            Some(ElseBranch::If(else_if_b)) => {
                // else if b
                match &else_if_b.else_branch {
                    Some(ElseBranch::If(else_if_c)) => {
                        // else if c
                        match &else_if_c.else_branch {
                            Some(ElseBranch::Else(nodes)) => {
                                assert_eq!(nodes.len(), 1);
                            }
                            _ => panic!("expected final ElseBranch::Else"),
                        }
                    }
                    _ => panic!("expected ElseBranch::If for 'c'"),
                }
            }
            _ => panic!("expected ElseBranch::If for 'b'"),
        }
    }

    #[test]
    fn test_if_let_some_condition() {
        let input = quote! { if let Some(x) = opt { "Has value" } };
        let result = parse_if(input).unwrap();

        assert_eq!(result.then_branch.len(), 1);
        assert!(result.else_branch.is_none());
    }

    #[test]
    fn test_if_let_with_else() {
        let input = quote! { if let Some(x) = opt { "Has value" } @else { "None" } };
        let result = parse_if(input).unwrap();

        assert_eq!(result.then_branch.len(), 1);
        assert!(matches!(result.else_branch, Some(ElseBranch::Else(_))));
    }

    #[test]
    fn test_complex_condition() {
        let input = quote! { if a && b || c { "Complex" } };
        let result = parse_if(input).unwrap();

        assert_eq!(result.then_branch.len(), 1);
    }

    #[test]
    fn test_comparison_condition() {
        let input = quote! { if count > 0 { "Has items" } };
        let result = parse_if(input).unwrap();

        assert_eq!(result.then_branch.len(), 1);
    }

    #[test]
    fn test_method_call_condition() {
        let input = quote! { if list.is_empty() { "Empty" } };
        let result = parse_if(input).unwrap();

        assert_eq!(result.then_branch.len(), 1);
    }

    #[test]
    fn test_if_with_expression_body() {
        let input = quote! { if show { (value) } };
        let result = parse_if(input).unwrap();

        assert_eq!(result.then_branch.len(), 1);
        assert!(matches!(result.then_branch[0], Node::Expression(_)));
    }

    #[test]
    fn test_error_missing_condition() {
        let input = quote! { if {} };
        let result = parse_if(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_missing_braces() {
        let input = quote! { if true "text" };
        let result = parse_if(input);
        assert!(result.is_err());
    }
}
