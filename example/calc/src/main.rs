// ビルド設定インポート
use depagerpp::builder::prelude::*;
use depagerpp::builder::env::Windows_X86_64;
use depagerpp::builder::target::{Compiler, Interpreter};

// 部品インポート
use depagerpp::dsl::prelude::*;
use depagerpp::dsl::synpart::{
    expr::{add, div, mul, sub},
    io::stdout,
    num::integer,
};

// 部品作成
#[dsl(synpart, extends = stdout)]
struct Print<T> {
    val: T,
}

impl DSL for Print {
    syntax_lalr! {
        print
            : "p" val
            | "print" val
            | "print" '(' val ')'
            ;
    }
}

// 構文定義
#[dsl(main)]
struct CalcLang;

impl DSL for CalcLang {
    syntax_lalr! {
        program
            : stmt
            | program stmt
            ;

        stmt
            : $Print(expr)
            ...
            ;

        ...
    }
}

fn main() {
    // ビルド実行
    langbuild!(CalcLang)
        .target(Compiler::<Windows_X86_64>::new())
        .target(Interpreter::<Windows_X86_64>::new())
        .build();
}
