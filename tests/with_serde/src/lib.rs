use compiler::load_dsl;

load_dsl!(MyDSL);

#[test]
fn test_with_serde() {
    let dsl = MyDSL::gen().unwrap();
    assert!(dsl.process("Hello A").is_ok());
    assert!(dsl.process("Hello A, B").is_ok());
    assert!(dsl.process("Hello A, B, C").is_ok());

    let deserialized = serde_json::to_string(&dsl).unwrap();

    let dsl = serde_json::from_str::<MyDSL>(&deserialized).unwrap();
    assert!(dsl.process("Hello A").is_ok());
    assert!(dsl.process("Hello A, B").is_ok());
    assert!(dsl.process("Hello A, B, C").is_ok());
}
