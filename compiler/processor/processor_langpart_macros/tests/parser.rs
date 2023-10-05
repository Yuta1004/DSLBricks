#[cfg(test)]
mod test {
    use langpart::parser::{ASyntax, LR1};
    use langpart::prelude::*;
    use processor_langpart_macros::*;

    #[lexer]
    #[derive(Debug)]
    enum TestToken {
        #[token(regex = "aaa")]
        TokenA,
        #[token(regex = "bbb")]
        TokenB,
        #[token(regex = "ccc")]
        TokenC,
    }

    #[parser(
        kind => LR1,
        token => TestToken,
        semantics => VoidSemantics,
        bnf =>
            STMT  : STMT2             $ STMT
                  ;
            STMT2 : "TokenA" "TokenB" $ AandB
                  | "TokenA" "TokenC" $ AandC
                  | "TokenB" "TokenC" $ BandC
                  ;
    )]
    pub enum TestSyntax {
        STMT,
        AandB,
        AandC,
        BandC,
    }

    struct VoidSemantics;

    impl<S, T> ASyntax<S, T> for VoidSemantics
    where
        S: Syntax<Self, T>,
        T: Token,
    {
        fn mapping(_: S, _: Vec<(Option<Box<Self>>, Option<&str>)>) -> anyhow::Result<Box<Self>> {
            Ok(Box::new(VoidSemantics {}))
        }
    }

    #[test]
    fn check_exists_methods() {
        TestToken::to_regex(&TestToken::TokenA);
        TestToken::ignore_str();
    }

    #[test]
    fn check_rule() {
        let syntax_rule = vec![
            (
                TestSyntax::STMT,
                Rule::from((RuleElem::nonterm("STMT"), vec![RuleElem::nonterm("STMT2")])),
            ),
            (
                TestSyntax::AandB,
                Rule::from((
                    RuleElem::nonterm("STMT2"),
                    vec![
                        RuleElem::term(TestToken::TokenA),
                        RuleElem::term(TestToken::TokenB),
                    ],
                )),
            ),
            (
                TestSyntax::AandC,
                Rule::from((
                    RuleElem::nonterm("STMT2"),
                    vec![
                        RuleElem::term(TestToken::TokenA),
                        RuleElem::term(TestToken::TokenC),
                    ],
                )),
            ),
            (
                TestSyntax::BandC,
                Rule::from((
                    RuleElem::nonterm("STMT2"),
                    vec![
                        RuleElem::term(TestToken::TokenB),
                        RuleElem::term(TestToken::TokenC),
                    ],
                )),
            ),
        ];
        for (syntax, rule) in syntax_rule {
            assert_eq!(TestSyntax::to_rule(&syntax), rule);
        }
    }
}
