use std::fmt::Display;

use crate::ir::{BlocklyIR, BlocklyIRComponent};

pub struct BlocklyInitString(String);

impl From<&BlocklyIR> for BlocklyInitString {
    fn from(ir: &BlocklyIR) -> Self {
        let body = match ir {
            BlocklyIR::NoConnection(body) => body,
            BlocklyIR::TopBottomConnections(body) => body,
            BlocklyIR::TopConnection(body) => body,
            BlocklyIR::BottomConnection(body) => body,
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

        BlocklyInitString(init)
    }
}

impl Display for BlocklyInitString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn message0(components: &[BlocklyIRComponent]) -> String {
    let (msgs, _) = components
        .iter()
        .fold((vec![], 0), |(mut msgs, args_cnt), component| {
            let (title, cnt) = match component {
                BlocklyIRComponent::Text { title } => (title, 1),
                BlocklyIRComponent::Variable { title, .. } => (title, 2),
                BlocklyIRComponent::TextInput { title, .. } => (title, 2),
                BlocklyIRComponent::BlockInput { title, .. } => (title, 2),
                BlocklyIRComponent::CheckBoxInput { title, ..} => (title, 2),
            };

            msgs.push(title.to_string());
            (args_cnt..args_cnt+cnt)
                .into_iter()
                .for_each(|idx| msgs.push(format!("%{}", idx+1)));

            (msgs, args_cnt+cnt)
        });
    msgs.join(" ")
}

fn args0(components: &[BlocklyIRComponent]) -> String {
    let into = |component: &BlocklyIRComponent| {
        match component {
            BlocklyIRComponent::Text { .. } => {
                format!(r#"
                {{
                    type: "input_dummy"
                }}
                "#)
            }
            BlocklyIRComponent::Variable { name, .. } => {
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
            BlocklyIRComponent::TextInput { name, .. } => {
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
            BlocklyIRComponent::BlockInput { name, .. } => {
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
            BlocklyIRComponent::CheckBoxInput { name, .. } => {
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

fn extjson0(ir: &BlocklyIR) -> String {
    match ir {
        BlocklyIR::NoConnection(_) => "",
        BlocklyIR::TopBottomConnections(_) => {
            r#"previousStatement: "null",nextStatement: "null""#
        }
        BlocklyIR::TopConnection(_) => {
            r#"previousStatement: "null""#
        }
        BlocklyIR::BottomConnection(_) => {
            r#"nextStatement: "null""#
        }
    }
    .to_string()
}
