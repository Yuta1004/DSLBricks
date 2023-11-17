use compiler::load_dsl;

load_dsl!(MyDSL);

#[test]
#[allow(unused_variables)]
fn test_default() {
    let dsl = MyDSL::gen().unwrap();
    // serde_json::to_string(&dsl).unwrap(); // => Erroe occurs
}
