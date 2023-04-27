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

// ビルド方法指示
/*
use depagerpp::build::prelude;
use depagerpp::build::target::Compiler;
use depagerpp::build::env::{ ARM64, Windows_X86_64 };

prod!(CalcLang, Compiler<Windows_ARM64>)
prod!(CalcLang, Compiler<Windows_X86_64>)

fn build() -> Result<Compiler> {
    let builder_arm64 = <Compiler<Windows_ARM64> as Target>::builder()
        .setup(...)
        .setup2(...)
        .setup3(...)
        .setup4(...)
        .build()?;

    let builder_x86_64 = <Compiler<Windows_X86_64> as Target>::builder()
        .setup(...)
        .setup2(...)
        .setup3(...)
        .setup4(...)
        .build()
}
 */

fn main() {
    println!("Hello, world!");
}
