use designer::design::syntax::Rule;

pub trait DSLBrickDesign {
    fn design(&self) -> Vec<Rule>;
}
