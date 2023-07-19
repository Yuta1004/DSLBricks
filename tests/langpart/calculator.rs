use depagerpp::langpart::parser::kind::LR1;
use depagerpp::langpart::parser::syntax::ASyntax;
use depagerpp::langpart::prelude::*;
use depagerpp::langpart::LangPart;
use depagerpp::langpart_macros::*;

#[lexer]
enum MyToken {
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
    token => MyToken,
    semantics => CalcNode,
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
enum MySyntax {
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
enum CalcNode {
    Plus(Box<CalcNode>, Box<CalcNode>),
    Minus(Box<CalcNode>, Box<CalcNode>),
    Mul(Box<CalcNode>, Box<CalcNode>),
    Div(Box<CalcNode>, Box<CalcNode>),
    Num(i32),
}

impl ASyntax<MySyntax, MyToken> for CalcNode {
    fn mapping(
        syntax: MySyntax,
        mut tokens: Vec<(Option<Box<Self>>, Option<&str>)>,
    ) -> anyhow::Result<Box<CalcNode>> {
        match syntax {
            MySyntax::ExprPlus => {
                let (term_a, _, term_b) = (tget!(tokens), tignore!(tokens), tget!(tokens));
                Ok(Box::new(CalcNode::Plus(term_a, term_b)))
            }
            MySyntax::ExprMinus => {
                let (term_a, _, term_b) = (tget!(tokens), tignore!(tokens), tget!(tokens));
                Ok(Box::new(CalcNode::Minus(term_a, term_b)))
            }
            MySyntax::TermMul => {
                let (term_a, _, term_b) = (tget!(tokens), tignore!(tokens), tget!(tokens));
                Ok(Box::new(CalcNode::Mul(term_a, term_b)))
            }
            MySyntax::TermDiv => {
                let (term_a, _, term_b) = (tget!(tokens), tignore!(tokens), tget!(tokens));
                Ok(Box::new(CalcNode::Div(term_a, term_b)))
            }
            MySyntax::Fact2Expr => {
                let (_, expr, _) = (tignore!(tokens), tget!(tokens), tignore!(tokens));
                Ok(expr)
            }
            MySyntax::Fact2Num => {
                let num = tget!(tokens, i32);
                Ok(Box::new(CalcNode::Num(num)))
            }
            MySyntax::Expr2Term => Ok(tget!(tokens)),
            MySyntax::Term2Fact => Ok(tget!(tokens)),
        }
    }
}

impl CalcNode {
    pub fn exec(&self) -> i32 {
        match self {
            CalcNode::Plus(a, b) => a.exec() + b.exec(),
            CalcNode::Minus(a, b) => a.exec() - b.exec(),
            CalcNode::Mul(a, b) => a.exec() * b.exec(),
            CalcNode::Div(a, b) => a.exec() / b.exec(),
            CalcNode::Num(num) => *num,
        }
    }
}

#[test]
fn calculator() {
    let lang = LangPart::<CalcNode, MySyntax, MyToken>::gen().unwrap();
    let check = |expr: &str, ans: i32| {
        assert_eq!(lang.process(expr).unwrap().exec(), ans);
    };

    check("10", 10);
    check("10 + 20", 30);
    check("10 * 20", 200);
    check("10 + 20 / 5", 14);
    check("10 / 5 + 20", 22);
    check("10 * 20 + 30 * 40", 1400);
    check("10 / 5 - 10 * 3 / 6", -3);
    check("(10)", 10);
    check("((10))", 10);
    check("(10+20) * (30-40)", -300);
    check("(10 + ((20 + 30) / 5))", 20);
}
