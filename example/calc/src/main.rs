// 部品インポート
/*
use depagerpp::dsl::prelude;
use depagerpp::dsl::syntax {
    num::integer,
    expr::{ add, sub, mul, div },
    io::stdout,
};
 */

// 部品作成
/*
assemble!(Print{ val: ... }, stdout::semantics,
    Print
        : "p" val
        | "print" val
        | "print" '(' val ')'
        ;
)

or

assemble_ns!(Print{ val: ... },
    Print
        : "p" val
        | "print" val
        | "print" "(" val ")"
        ;
)

impl Semantics for Print {
    fn ... {}
}
 */

// 構文定義
/*
langdef!(
    CalcLang,
    program
        : stmt
        | program stmt
        ;

    stmt
        | Print(expr) ";"
    ...
)
 */

use depagerpp::builder::env::Windows_X86_64;
use depagerpp::builder::target::Compiler;
use depagerpp::builder::Builder;

fn main() {
    Builder::new()
        .add(Compiler::<Windows_X86_64>::new())
        .build()
}
