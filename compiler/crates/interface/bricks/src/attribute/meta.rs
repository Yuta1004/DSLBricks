pub trait DSLBrickMeta {
    fn name(&self) -> &'static str;
    fn start(&self) -> &'static str;
    fn components(&self) -> &[&'static str];
}
