use std::fmt::Display;

use crate::ir::{BlocklyIR, BlocklyIRComponent};

pub struct BlocklyInitString(String);

impl From<&BlocklyIR> for BlocklyInitString {
    fn from(ir: &BlocklyIR) -> Self {
        let message0 = message0(&ir.components);
        let args0 = args0(&ir.components);
        let init = format!(r#"
            function() {{
                this.jsonInit({{
                    "type": "{}",
                    "message0": "{}",
                    "args0": [{}],
                    "colour": 200,
                    "tooltop": "",
                    "helpUrl": ""
                }})
            }}
        "#, ir.ty, message0, args0);

        BlocklyInitString(init)
    }
}

impl Display for BlocklyInitString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn message0(components: &[BlocklyIRComponent]) -> String {
    let mut msg_elems = vec![];
    let mut arg_cnt = 0;
    for component in components {
        match component {
            BlocklyIRComponent::Text { title } => {
                msg_elems.push(title.to_owned());
                msg_elems.push(format!("%{}", arg_cnt + 1));
                arg_cnt += 1;
            }
            BlocklyIRComponent::Variable { title } => {
                msg_elems.push(title.to_owned());
                msg_elems.push(format!("%{}", arg_cnt + 1));
                msg_elems.push(format!("%{}", arg_cnt + 2));
                arg_cnt += 2;
            }
            BlocklyIRComponent::TextInput { title } => {
                msg_elems.push(title.to_owned());
                msg_elems.push(format!("%{}", arg_cnt + 1));
                msg_elems.push(format!("%{}", arg_cnt + 2));
                arg_cnt += 2;
            }
            BlocklyIRComponent::BlockInput { title } => {
                msg_elems.push(title.to_owned());
                msg_elems.push(format!("%{}", arg_cnt + 1));
                msg_elems.push(format!("%{}", arg_cnt + 2));
                arg_cnt += 2;
            }
        };
    }
    msg_elems.join(" ")
}

fn args0(components: &[BlocklyIRComponent]) -> String {
    let into = |component: &BlocklyIRComponent| {
        match component {
            BlocklyIRComponent::Text { .. } => {
                format!(r#"
                {{
                    "type": "input_dummy"
                }}
                "#)
            }
            BlocklyIRComponent::Variable { .. } => {
                format!(r#"
                {{
                    "type": "field_variable",
                    "name": "NAME",
                    "variable": "var"
                }},
                {{
                    "type": "input_dummy"
                }}
                "#)
            }
            BlocklyIRComponent::TextInput { .. } => {
                format!(r#"
                {{
                    "type": "field_input",
                    "name": "NAME",
                    "text": "Executable"
                }},
                {{
                    "type": "input_dummy"
                }}
                "#)
            }
            BlocklyIRComponent::BlockInput { .. } => {
                format!(r#"
                {{
                    "type": "input_dummy"
                }},
                {{
                    "type": "input_statement",
                    "name": "NAME"
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
