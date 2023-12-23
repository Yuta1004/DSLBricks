mod string;

use std::fmt::Display;

use string::ContentString;

use crate::ir;

pub struct ToolBox {
    name: String,
    contents: Vec<ContentString>,
}

impl<T: Into<String>> From<(T, &[ir::Block])> for ToolBox {
    fn from((name, irs): (T, &[ir::Block])) -> Self {
        ToolBox {
            name: name.into(),
            contents: irs.into_iter().map(ContentString::from).collect(),
        }
    }
}

impl Display for ToolBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let contents = self
            .contents
            .iter()
            .map(|content| format!("{}", content))
            .collect::<Vec<String>>()
            .join(",");

        write!(f, r#"
            {{
                kind: "category",
                name: "{}",
                colour: "100",
                contents: [{}]
            }}
        "#, self.name, contents)
    }
}

#[cfg(test)]
mod test {
    use super::ToolBox;
    use crate::ir;

    #[test]
    fn simple() {
        let irs = vec![
            ir::Block::new_no_connection("test1", vec![]),
            ir::Block::new_no_connection("test2", vec![]),
            ir::Block::new_no_connection("test3", vec![]),
        ];
        let toolbox = ToolBox::from(("Group", irs.as_slice()));
        let _ = format!("{}", toolbox);
    }
}
