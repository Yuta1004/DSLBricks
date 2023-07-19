#[cfg(test)]
mod test {
    use langpart::prelude::*;
    use depagerpp_langpart_macros::*;

    #[lexer]
    enum TestToken {
        #[token(regex = "aaa")]
        TokenA,
        #[token(regex = "bbb")]
        TokenB,
        #[token(regex = "ccc")]
        TokenC,
    }

    #[test]
    fn check_exists_methods() {
        TestToken::to_regex(&TestToken::TokenA);
        TestToken::ignore_str();
    }

    #[test]
    fn check_regex() {
        let token_regex = vec![
            (TestToken::TokenA, "^aaa"),
            (TestToken::TokenB, "^bbb"),
            (TestToken::TokenC, "^ccc"),
        ];
        for (token, regex) in token_regex {
            assert_eq!(TestToken::to_regex(&token), regex);
        }
    }
}
