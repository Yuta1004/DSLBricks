use std::fmt::Display;

use crate::ir;

pub struct InitString(String);

impl From<&ir::Block> for InitString {
    fn from(ir: &ir::Block) -> Self {
        let body = match ir {
            ir::Block::NoConnection(body) => body,
            ir::Block::TopBottomConnections(body) => body,
            ir::Block::TopConnection(body) => body,
            ir::Block::BottomConnection(body) => body,
        };

        let message0 = message0(&body.components);
        let args0 = args0(&body.components);
        let extjson0 = extjson0(ir);
        let init = format!(r#"
            function() {{
                this.jsonInit({{
                    type: "{}",
                    message0: "{}",
                    args0: [{}],
                    colour: 200,
                    tooltop: "",
                    helpUrl: "",
                    {}
                }})
            }}
        "#, body.ty, message0, args0, extjson0);

        InitString(init)
    }
}

impl Display for InitString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn message0(components: &[ir::BlockComponent]) -> String {
    let (msgs, _) = components
        .iter()
        .fold((vec![], 0), |(mut msgs, args_cnt), component| {
            let (title, cnt) = match component {
                ir::BlockComponent::Text { title } => (title, 1),
                ir::BlockComponent::Variable { title, .. } => (title, 2),
                ir::BlockComponent::TextInput { title, .. } => (title, 2),
                ir::BlockComponent::BlockInput { title, .. } => (title, 2),
                ir::BlockComponent::CheckBoxInput { title, ..} => (title, 2),
            };

            msgs.push(title.to_string());
            (args_cnt..args_cnt+cnt)
                .into_iter()
                .for_each(|idx| msgs.push(format!("%{}", idx+1)));

            (msgs, args_cnt+cnt)
        });
    msgs.join(" ")
}

fn args0(components: &[ir::BlockComponent]) -> String {
    let into = |component: &ir::BlockComponent| {
        match component {
            ir::BlockComponent::Text { .. } => {
                format!(r#"
                {{
                    type: "input_dummy"
                }}
                "#)
            }
            ir::BlockComponent::Variable { name, .. } => {
                format!(r#"
                {{
                    type: "field_variable",
                    name: "{}",
                }},
                {{
                    type: "input_dummy"
                }}
                "#, name)
            }
            ir::BlockComponent::TextInput { name, .. } => {
                format!(r#"
                {{
                    type: "field_input",
                    name: "{name}",
                    text: ""
                }},
                {{
                    type: "input_dummy"
                }}
                "#)
            }
            ir::BlockComponent::BlockInput { name, .. } => {
                format!(r#"
                {{
                    type: "input_dummy"
                }},
                {{
                    type: "input_statement",
                    name: "{name}"
                }}
                "#)
            },
            ir::BlockComponent::CheckBoxInput { name, .. } => {
                format!(r#"
                {{
                    type: "field_checkbox",
                    name: "{name}",
                    checked: false
                }},
                {{
                    type: "input_dummy"
                }}
                "#)
            }
        }
    };

    components
        .iter()
        .map(into)
        .collect::<Vec<String>>()
        .join(",")
}

fn extjson0(ir: &ir::Block) -> String {
    match ir {
        ir::Block::NoConnection(_) => "",
        ir::Block::TopBottomConnections(_) => {
            r#"previousStatement: "null",nextStatement: "null""#
        }
        ir::Block::TopConnection(_) => {
            r#"previousStatement: "null""#
        }
        ir::Block::BottomConnection(_) => {
            r#"nextStatement: "null""#
        }
    }
    .to_string()
}
