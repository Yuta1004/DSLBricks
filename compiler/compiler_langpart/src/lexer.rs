use std::hash::Hash;
use std::marker::PhantomData;

use regex::{Regex, RegexSet};
use strum::IntoEnumIterator;

pub trait Token: IntoEnumIterator + Copy + Hash + Eq {
    fn to_regex(token: &Self) -> &'static str;
    fn ignore_str() -> &'static str;
}

pub struct Lexer<T: Token> {
    ty: PhantomData<T>,
    regex_set: RegexSet,
    regex_map: Vec<(Regex, T)>,
    regex_istr: Regex,
}

impl<T: Token + 'static> Lexer<T> {
    pub fn new() -> anyhow::Result<Lexer<T>> {
        let regex_set: Vec<&str> = T::iter().map(|token| T::to_regex(&token)).collect();
        let regex_set = RegexSet::new(regex_set)?;

        let regex_map: Vec<(Regex, T)> = T::iter()
            .map(|token| (Regex::new(T::to_regex(&token)).unwrap(), token))
            .collect();

        let regex_istr = Regex::new(T::ignore_str())?;

        Ok(Lexer {
            ty: PhantomData,
            regex_set,
            regex_map,
            regex_istr,
        })
    }

    pub fn lex<'a>(&self, input: &'a str) -> Box<dyn Iterator<Item = (&'a str, T)> + 'a> {
        let regex_set = self.regex_set.clone();
        let regex_map = self.regex_map.clone();
        let regex_istr = self.regex_istr.clone();
        Box::new(LexDriver::<'a, T>::new(
            regex_set, regex_map, regex_istr, input,
        ))
    }
}

struct LexDriver<'a, T: Token> {
    ty: PhantomData<T>,
    regex_set: RegexSet,
    regex_map: Vec<(Regex, T)>,
    regex_istr: Regex,
    input: &'a str,
}

impl<'a, T: Token> LexDriver<'a, T> {
    fn new(
        regex_set: RegexSet,
        regex_map: Vec<(Regex, T)>,
        regex_istr: Regex,
        input: &'a str,
    ) -> Self {
        LexDriver {
            ty: PhantomData,
            regex_set,
            regex_map,
            regex_istr,
            input,
        }
    }
}

impl<'a, T: Token> Iterator for LexDriver<'a, T> {
    type Item = (&'a str, T);

    fn next(&mut self) -> Option<Self::Item> {
        // Skip spaces
        match self.regex_istr.find(&self.input) {
            Some(s) => self.input = &self.input[s.len()..],
            None => {}
        }

        // Found the token
        let (s, token): (&str, &T) = self
            .regex_set
            .matches(&self.input)
            .into_iter()
            .map(|idx| &self.regex_map[idx])
            .map(|(regex, token)| (regex.find(&self.input).unwrap().as_str(), token))
            .next()?;
        self.input = &self.input[s.len()..];

        Some((s, *token))
    }
}

#[cfg(test)]
mod test {
    use super::Lexer;
    use crate::lexer::Token;
    use crate::prelude::*;

    #[derive(EnumIter, Clone, Copy, Debug, Hash, PartialEq, Eq)]
    enum TestToken {
        Num,
        Plus,
    }

    impl Token for TestToken {
        fn to_regex(token: &Self) -> &'static str {
            match token {
                TestToken::Num => r"^[1-9][0-9]*",
                TestToken::Plus => r"^\+",
            }
        }

        fn ignore_str() -> &'static str {
            r"^[ \t\n]+"
        }
    }

    fn gen_lexer() -> Lexer<TestToken> {
        Lexer::<TestToken>::new().unwrap()
    }

    fn check<'a>(
        expected: &Vec<(&str, TestToken)>,
        lexer: impl Iterator<Item = (&'a str, TestToken)>,
    ) -> bool {
        lexer
            .into_iter()
            .zip(expected.iter())
            .all(|(a, b)| a.0 == b.0 && a.1 == b.1)
    }

    #[test]
    fn input_ok_1() {
        let expected = vec![
            ("10", TestToken::Num),
            ("+", TestToken::Plus),
            ("20", TestToken::Num),
        ];
        let lexer = gen_lexer();

        assert!(check(&expected, lexer.lex("10+20")));
    }

    #[test]
    fn input_ok_2() {
        let expected = vec![
            ("10", TestToken::Num),
            ("+", TestToken::Plus),
            ("20", TestToken::Num),
        ];
        let lexer = gen_lexer();

        assert!(check(&expected, lexer.lex("            10 +      20     ")));
    }

    #[test]
    fn input_ok_3() {
        let expected = vec![
            ("10", TestToken::Num),
            ("+", TestToken::Plus),
            ("20", TestToken::Num),
        ];
        let lexer = gen_lexer();

        assert!(check(
            &expected,
            lexer.lex("            10 +      20ffff30 - 40 * 50")
        ));
    }

    #[test]
    fn input_ok_4() {
        let expected = vec![
            ("10", TestToken::Num),
            ("+", TestToken::Plus),
            ("20", TestToken::Num),
        ];
        let lexer = gen_lexer();

        assert!(check(&expected, lexer.lex("10 + 20")));
        assert!(check(&expected, lexer.lex("10    + 20")));
        assert!(check(&expected, lexer.lex("   10    + 20   ")));
    }
}
