mod pimpl;
pub mod rule;
pub mod syntax;
pub mod prelude;

use std::fmt::Display;
use std::marker::PhantomData;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use lexer::{Token, LexIterator};

use pimpl::ParserImpl;
pub use pimpl::LR1;
use syntax::{ASyntax, Syntax};

#[derive(Debug, Error, Serialize, Deserialize)]
pub struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ParseError {
    fn from(remain: String) -> Self {
        ParseError(remain)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parser<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token,
{
    syntax: PhantomData<S>,
    p_impl: S::Parser,
}

#[allow(clippy::new_without_default)]
impl<A, S, T> Parser<A, S, T>
where
    A: ASyntax<S, T>,
    S: Syntax<A, T>,
    T: Token,
{
    pub fn new() -> anyhow::Result<Parser<A, S, T>> {
        Ok(Parser {
            syntax: PhantomData,
            p_impl: S::Parser::setup()?,
        })
    }

    pub fn parse<'a, 'b>(
        &self,
        lexer: &'a mut impl LexIterator<'b, T>,
    ) -> anyhow::Result<Box<A>> {
        self.p_impl.parse(lexer)
    }
}

// #[cfg(test)]
// mod test {
//     use serde::{Serialize, Deserialize};

//     use super::*;
//     use super::prelude::*;

//     #[derive(Serialize, Deserialize)]
//     pub struct VoidSemantics;

//     impl<S, T> ASyntax<S, T> for VoidSemantics
//     where
//         S: Syntax<Self, T>,
//         T: Token,
//     {
//         fn mapping(_: S, _: Vec<(Option<Box<Self>>, Option<&str>)>) -> anyhow::Result<Box<Self>> {
//             Ok(Box::new(VoidSemantics {}))
//         }
//     }

//     #[derive(EnumIter, Clone, Copy, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
//     enum TestToken {
//         Num,
//         Plus,
//         Minus,
//         Mul,
//         Div,
//         BracketA,
//         BracketB,
//     }

//     impl Token for TestToken {
//         fn to_regex(token: &Self) -> &'static str {
//             match token {
//                 TestToken::Num => r"^[1-9][0-9]*",
//                 TestToken::Plus => r"^\+",
//                 TestToken::Minus => r"^-",
//                 TestToken::Mul => r"^\*",
//                 TestToken::Div => r"^/",
//                 TestToken::BracketA => r"^\(",
//                 TestToken::BracketB => r"^\)",
//             }
//         }

//         fn ignore_str() -> &'static str {
//             r"^[ \t\n]+"
//         }
//     }

//     #[derive(EnumIter, Clone, Copy, Debug, Serialize, Deserialize)]
//     pub enum TestSyntax {
//         ExprPlus,
//         ExprMinus,
//         Expr2Term,
//         TermMul,
//         TermDiv,
//         Term2Fact,
//         Fact2Expr,
//         Fact2Num,
//     }

//     impl Syntax<VoidSemantics, TestToken> for TestSyntax {
//         type Parser = LR1<VoidSemantics, TestSyntax, TestToken>;

//         fn to_rule(&self) -> Rule<TestToken> {
//             let expr_plus = Rule::from((
//                 RuleElem::nonterm("expr"),
//                 vec![
//                     RuleElem::nonterm("expr"),
//                     RuleElem::term(TestToken::Plus),
//                     RuleElem::nonterm("term"),
//                 ],
//             ));

//             let expr_minus = Rule::from((
//                 RuleElem::nonterm("expr"),
//                 vec![
//                     RuleElem::nonterm("expr"),
//                     RuleElem::term(TestToken::Minus),
//                     RuleElem::nonterm("term"),
//                 ],
//             ));

//             let expr_2_term = Rule::<TestToken>::from((
//                 RuleElem::nonterm("expr"),
//                 vec![RuleElem::nonterm("term")],
//             ));

//             let term_mul = Rule::from((
//                 RuleElem::nonterm("term"),
//                 vec![
//                     RuleElem::nonterm("term"),
//                     RuleElem::term(TestToken::Mul),
//                     RuleElem::nonterm("fact"),
//                 ],
//             ));

//             let term_div = Rule::from((
//                 RuleElem::nonterm("term"),
//                 vec![
//                     RuleElem::nonterm("term"),
//                     RuleElem::term(TestToken::Div),
//                     RuleElem::nonterm("fact"),
//                 ],
//             ));

//             let term_2_fact = Rule::<TestToken>::from((
//                 RuleElem::nonterm("term"),
//                 vec![RuleElem::nonterm("fact")],
//             ));

//             let fact_2_expr = Rule::from((
//                 RuleElem::nonterm("fact"),
//                 vec![
//                     RuleElem::term(TestToken::BracketA),
//                     RuleElem::nonterm("expr"),
//                     RuleElem::term(TestToken::BracketB),
//                 ],
//             ));

//             let fact_2_num = Rule::from((
//                 RuleElem::nonterm("fact"),
//                 vec![RuleElem::term(TestToken::Num)],
//             ));

//             match self {
//                 TestSyntax::ExprPlus => expr_plus,
//                 TestSyntax::ExprMinus => expr_minus,
//                 TestSyntax::Expr2Term => expr_2_term,
//                 TestSyntax::TermMul => term_mul,
//                 TestSyntax::TermDiv => term_div,
//                 TestSyntax::Term2Fact => term_2_fact,
//                 TestSyntax::Fact2Expr => fact_2_expr,
//                 TestSyntax::Fact2Num => fact_2_num,
//             }
//         }
//     }

//     #[test]
//     fn serde() {
//         let parser = Parser::<VoidSemantics, TestSyntax, TestToken>::new().unwrap();
//         panic!("{}", serde_json::to_string(&parser).unwrap());
//     }
// }
