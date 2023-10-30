use blockdsl::lib::dev::dsl::expr_dsl;

#[test]
fn calculator() {
    let dsl = expr_dsl();
    let check = |expr: &str, ans: i32| {
        assert_eq!(dsl.process(expr).unwrap().exec(), ans);
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
