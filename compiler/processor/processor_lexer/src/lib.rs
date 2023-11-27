use std::hash::Hash;
use std::marker::PhantomData;

use regex::{Regex, RegexSet};

#[cfg(feature = "with-serde")]
use serde::{Deserialize, Serialize};

use util_macros::cfg_where;

#[cfg_where(feature = "with-serde", Self: Serialize)]
pub trait TokenSet
where
    Self: Copy + Clone + Hash + Eq,
{
    // for Enum
    fn iter() -> Box<dyn Iterator<Item = Self>>;

    // for Variants
    fn to_regex(token: &Self) -> &'static str;
    fn ignore_str() -> &'static str;
}

#[derive(Debug, Copy, Clone)]
pub struct Token<'a, T: TokenSet> {
    pub kind: T,
    pub raw: &'a str,
    pub pos: (u32, u32),
}

impl<'a, T: TokenSet> Token<'a, T> {
    fn new(kind: T, raw: &'a str, pos: (u32, u32)) -> Self {
        Token { kind, raw, pos }
    }
}

#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Lexer<T: TokenSet>(PhantomData<T>);

impl<T: TokenSet + 'static> Lexer<T> {
    pub fn new() -> anyhow::Result<Lexer<T>> {
        let regex_set: Vec<&str> = T::iter().map(|token| T::to_regex(&token)).collect();
        let _ = RegexSet::new(regex_set)?;
        let _ = Regex::new(T::ignore_str())?;

        Ok(Lexer(PhantomData))
    }

    pub fn lex<'a>(&self, input: &'a str) -> impl LexIterator<'a, T> + 'a {
        let regex_set: Vec<&str> = T::iter().map(|token| T::to_regex(&token)).collect();
        let regex_set = RegexSet::new(regex_set).unwrap();

        let regex_map: Vec<(Regex, T)> = T::iter()
            .map(|token| (Regex::new(T::to_regex(&token)).unwrap(), token))
            .collect();

        let regex_istr = Regex::new(T::ignore_str()).unwrap();

        LexDriver::<'a, T>::new(regex_set, regex_map, regex_istr, input)
    }
}

pub trait LexIterator<'a, T: TokenSet>
where
    Self: Iterator<Item = Token<'a, T>>,
{
    fn pos(&self) -> (u32, u32);
    fn remain(&self) -> Option<&'a str>;
}

struct LexDriver<'a, T: TokenSet> {
    // PhantomData
    ty: PhantomData<T>,

    // Regex
    regex_set: RegexSet,
    regex_map: Vec<(Regex, T)>,
    regex_istr: Regex,

    // State
    input: &'a str,
    pos: (u32, u32),
}

impl<'a, T: TokenSet> LexDriver<'a, T> {
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
            pos: (0, 0),
        }
    }
}

impl<'a, T: TokenSet> LexIterator<'a, T> for LexDriver<'a, T> {
    fn pos(&self) -> (u32, u32) {
        self.pos
    }

    fn remain(&self) -> Option<&'a str> {
        match self.input {
            "" => None,
            s => Some(s),
        }
    }
}

impl<'a, T: TokenSet> Iterator for LexDriver<'a, T> {
    type Item = Token<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip spaces
        if let Some(acc_s) = self.regex_istr.find(self.input) {
            self.update_state(acc_s.as_str());
        }

        // Found the token
        let (token, acc_s): (T, &str) = self
            .regex_set
            .matches(self.input)
            .into_iter()
            .map(|idx| &self.regex_map[idx])
            .map(|(regex, token)| (*token, regex.find(self.input).unwrap().as_str()))
            .next()?;
        let pos = self.pos;
        self.update_state(acc_s);

        Some(Token::new(token, acc_s, pos))
    }
}

impl<'a, T: TokenSet> LexDriver<'a, T> {
    fn update_state(&mut self, acc_s: &str) {
        let (mut rows, mut cols) = self.pos;
        for c in acc_s.chars() {
            match c {
                '\n' => {
                    rows += 1;
                    cols = 0;
                }
                _ => {
                    cols += 1;
                }
            }
        }

        self.input = &self.input[acc_s.len()..];
        self.pos = (rows, cols);
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    use super::{Lexer, Token, TokenSet};

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
    enum TestToken {
        Num,
        Plus,
    }

    impl TokenSet for TestToken {
        fn iter() -> Box<dyn Iterator<Item = Self>> {
            Box::new(vec![TestToken::Num, TestToken::Plus].into_iter())
        }

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
        expected: &Vec<(TestToken, &'a str, (u32, u32))>,
        lexer: impl Iterator<Item = Token<'a, TestToken>>,
    ) -> bool {
        lexer
            .into_iter()
            .zip(expected.iter())
            .all(|(a, b)| a.kind == b.0 && a.raw == b.1 && a.pos == b.2)
    }

    #[test]
    fn input_ok_1() {
        let expected = vec![
            (TestToken::Num, "10", (0, 0)),
            (TestToken::Plus, "+", (0, 2)),
            (TestToken::Num, "20", (0, 3)),
        ];
        let lexer = gen_lexer();

        assert!(check(&expected, lexer.lex("10+20")));
    }

    #[test]
    fn input_ok_2() {
        let expected = vec![
            (TestToken::Num, "10", (0, 12)),
            (TestToken::Plus, "+", (0, 15)),
            (TestToken::Num, "20", (1, 6)),
        ];
        let lexer = gen_lexer();

        assert!(check(
            &expected,
            lexer.lex("            10 +\n      20     ")
        ));
    }

    #[test]
    fn input_ok_3() {
        let expected = vec![
            (TestToken::Num, "10", (0, 12)),
            (TestToken::Plus, "+", (0, 15)),
            (TestToken::Num, "20", (1, 6)),
        ];
        let lexer = gen_lexer();

        assert!(check(
            &expected,
            lexer.lex("            10 +\n      20ffff30 - 40 * 50")
        ));
    }

    #[test]
    fn input_ok_4() {
        let expected = vec![
            (TestToken::Num, "10", (0, 0)),
            (TestToken::Plus, "+", (0, 3)),
            (TestToken::Num, "20", (0, 5)),
        ];
        let lexer = gen_lexer();

        assert!(check(&expected, lexer.lex("10 + 20")));
        assert!(check(&expected, lexer.lex("10 + 20")));
        assert!(check(&expected, lexer.lex("10 + 20")));
    }
}
