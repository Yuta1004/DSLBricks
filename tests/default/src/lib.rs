use compiler::load_dsl;

load_dsl!();

#[test]
#[allow(unused_variables)]
fn test_default() {
    let dsl = DSL::gen().unwrap();
    // serde_json::to_string(&dsl).unwrap(); // => Erroe occurs
}
