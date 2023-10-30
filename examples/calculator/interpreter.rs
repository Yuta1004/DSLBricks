use blockdsl::driver::interpreter::Interpreter;
use blockdsl::dsl::lexer::Token;
use blockdsl::dsl::parser::{tget, tignore};
use blockdsl::dsl::parser::rule::{Rule, RuleElem};
use blockdsl::dsl::parser::{ASyntax, LR1, Syntax};
use blockdsl::dsl::macros::*;
use blockdsl::dsl::prelude::*;
use blockdsl::dsl::DSL;

#[lexer]
pub enum ExprToken {
    #[token(regex = r"[1-9][0-9]*")]
    Num,
    #[token(regex = r"\+")]
    Plus,
    #[token(regex = r"-")]
    Minus,
    #[token(regex = r"\*")]
    Mul,
    #[token(regex = r"/")]
    Div,
    #[token(regex = r"\(")]
    BracketA,
    #[token(regex = r"\)")]
    BracketB,
}

#[parser(
    kind => LR1,
    token => ExprToken,
    semantics => ExprNode,
    bnf =>
        expr : expr "Plus" term     $ ExprPlus
             | expr "Minus" term    $ ExprMinus
             | term                 $ Expr2Term
             ;
        term : term "Mul" fact      $ TermMul
             | term "Div" fact      $ TermDiv
             | fact                 $ Term2Fact
             ;
        fact : "BracketA" expr "BracketB" $ Fact2Expr
             | "Num"                      $ Fact2Num
             ;
)]
pub enum ExprSyntax {
    ExprPlus,
    ExprMinus,
    Expr2Term,
    TermMul,
    TermDiv,
    Term2Fact,
    Fact2Expr,
    Fact2Num,
}

#[derive(Debug)]
pub enum ExprNode {
    Plus(Box<ExprNode>, Box<ExprNode>),
    Minus(Box<ExprNode>, Box<ExprNode>),
    Mul(Box<ExprNode>, Box<ExprNode>),
    Div(Box<ExprNode>, Box<ExprNode>),
    Num(i32),
}

impl ASyntax<ExprSyntax, ExprToken> for ExprNode {
    fn mapping(
        syntax: ExprSyntax,
        mut tokens: Vec<(Option<Box<Self>>, Option<&str>)>,
    ) -> anyhow::Result<Box<ExprNode>> {
        match syntax {
            ExprSyntax::ExprPlus => {
                let (term_a, _, term_b) = (tget!(tokens), tignore!(tokens), tget!(tokens));
                Ok(Box::new(ExprNode::Plus(term_a, term_b)))
            }
            ExprSyntax::ExprMinus => {
                let (term_a, _, term_b) = (tget!(tokens), tignore!(tokens), tget!(tokens));
                Ok(Box::new(ExprNode::Minus(term_a, term_b)))
            }
            ExprSyntax::TermMul => {
                let (term_a, _, term_b) = (tget!(tokens), tignore!(tokens), tget!(tokens));
                Ok(Box::new(ExprNode::Mul(term_a, term_b)))
            }
            ExprSyntax::TermDiv => {
                let (term_a, _, term_b) = (tget!(tokens), tignore!(tokens), tget!(tokens));
                Ok(Box::new(ExprNode::Div(term_a, term_b)))
            }
            ExprSyntax::Fact2Expr => {
                let (_, expr, _) = (tignore!(tokens), tget!(tokens), tignore!(tokens));
                Ok(expr)
            }
            ExprSyntax::Fact2Num => {
                let num = tget!(tokens, i32);
                Ok(Box::new(ExprNode::Num(num)))
            }
            ExprSyntax::Expr2Term => Ok(tget!(tokens)),
            ExprSyntax::Term2Fact => Ok(tget!(tokens)),
        }
    }
}

impl ExprNode {
    pub fn eval(&self) -> i32 {
        match self {
            ExprNode::Plus(a, b) => a.eval() + b.eval(),
            ExprNode::Minus(a, b) => a.eval() - b.eval(),
            ExprNode::Mul(a, b) => a.eval() * b.eval(),
            ExprNode::Div(a, b) => a.eval() / b.eval(),
            ExprNode::Num(num) => *num,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let dsl = DSL::<ExprNode, ExprSyntax, ExprToken>::gen()?;
    Interpreter::new(dsl).exec()
}
