use std::fmt::Display;

use tauri::InvokeError;

use blockly::back::ir::{BlocklyIR, BlocklyIRComponent};
use blockly::back::parse_xml;

#[tauri::command]
pub fn convert_xml(xml: &str) -> Result<String, InvokeError> {
    match parse_xml(xml) {
        Ok(irs) => Ok(format!("{}", DSLBuildFunc::from(irs))),
        Err(err) => Err(InvokeError::from(format!("{}", err))),
    }
}

struct DSLBuildFunc {
    declares: Vec<BrickDeclare>,
}

impl From<Vec<BlocklyIR>> for DSLBuildFunc {
    fn from(irs: Vec<BlocklyIR>) -> Self {
        DSLBuildFunc {
            declares: irs.into_iter().flat_map(BrickDeclare::from).collect(),
        }
    }
}

impl Display for DSLBuildFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let declares = self.declares
            .iter()
            .map(|declare| format!("{}", declare))
            .collect::<Vec<String>>()
            .join("\n\n");
        write!(f, "#[combine_bricks]\nfn main() {{\n{}\n}}", declares)
    }
}

struct BrickDeclare {
    ty: String,
    var: String,
    fields: Vec<BrickDeclareField>,
}

impl BrickDeclare {
    fn from(ir: BlocklyIR) -> Option<Self> {
        let (mut var, mut fields) = (None, vec![]);
        for component in ir.components {
            match component  {
                BlocklyIRComponent::Field { name, value } => {
                    match name.as_str() {
                        "variable" => { var = Some(value); },
                        _ => {},
                    }
                }
                BlocklyIRComponent::Blocks { name, blocks } => {
                    fields.push(BrickDeclareField::from(blocks));
                }
            }
        }

        if let Some(var) = var {
            let ty = ir.ty;
            Some(BrickDeclare { ty, var, fields })
        } else {
            None
        }
    }
}

impl Display for BrickDeclare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.fields.len() > 0 {
            let fields = self.fields
                .iter()
                .map(|field| format!("{}", field))
                .collect::<Vec<String>>()
                .join("\n");
            write!(f, "    let {} = {} {{\n{}\n    }};", self.var, self.ty, fields)
        } else {
            write!(f, "    let {} = {} {{ }};", self.var, self.ty)
        }
    }
}

struct BrickDeclareField {
    name: String,
    uses: Vec<String>,
}

impl From<Vec<BlocklyIR>> for BrickDeclareField {
    fn from(irs: Vec<BlocklyIR>) -> Self {
        BrickDeclareField {
            name: "dummy".to_string(),
            uses: vec![],
        }
    }
}

impl Display for BrickDeclareField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.uses.len() != 1 {
            write!(f, "        {}: [{}],", self.name, self.uses.join(","))
        } else {
            write!(f, "        {}: {},", self.name, self.uses[0])
        }
    }
}
