use std::fmt::Display;

use tauri::InvokeError;

use blockly::back::ir::BlocklyIR;
use blockly::back::parse_str;

#[tauri::command]
pub fn genrs(xml: &str) -> Result<String, InvokeError> {
    match parse_str(xml) {
        Ok(irs) => Ok(format!("{}", DSLBuildFunc::from(irs))),
        Err(err) => Err(InvokeError::from(format!("{}", err))),
    }
}

struct DSLBuildFunc {
    uses: Vec<String>,
    declares: Vec<BrickDeclare>,
}

impl From<Vec<BlocklyIR>> for DSLBuildFunc {
    fn from(irs: Vec<BlocklyIR>) -> Self {
        let uses = irs
            .iter()
            .map(|ir| {
                let mut ty = ir.r#type.split(".");
                let _ = ty.next();
                format!("use catalog::{};", ty.collect::<Vec<&str>>().join("::"))
            })
            .collect::<Vec<String>>();

        DSLBuildFunc {
            uses,
            declares: irs.into_iter().flat_map(BrickDeclare::from).collect(),
        }
    }
}

impl Display for DSLBuildFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let uses = self.uses.join("\n");
        let declares = self.declares
            .iter()
            .map(|declare| format!("{}", declare))
            .collect::<Vec<String>>()
            .join("\n\n");

        // Use (prelude, macro)
        writeln!(f, "// Prelude, Macro")?;
        writeln!(f, "use catalog::prelude::*;")?;
        writeln!(f, "use catalog::macros::combine_bricks;")?;

        // Use (bricks)
        if uses.len() > 0 {
            writeln!(f, "// Bricks")?;
            writeln!(f, "{}", uses)?;
        }

        // Main func
        if declares.len() > 0 {
            writeln!(f, "\n#[combine_bricks]\nfn main() {{\n{}\n}}", declares)
        } else {
            writeln!(f, "\n#[combine_bricks]\nfn main() {{ }}")
        }
    }
}

struct BrickDeclare {
    ty: String,
    var: String,
    fields: Vec<BrickDeclareField>,
}

impl BrickDeclare {
    fn from(ir: BlocklyIR) -> Option<Self> {
        if let Some(var) = ir.fields.get("variable") {
            let ty = ir.r#type.split('.').last().unwrap().to_string();
            let var = var.to_string();
            let fields = ir.blocks.iter().map(From::from).collect();
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
    variables: Vec<String>,
}

impl From<(&String, &Vec<BlocklyIR>)> for BrickDeclareField {
    fn from((name, irs): (&String, &Vec<BlocklyIR>)) -> Self {
        let name = name.to_string();
        let variables = irs
            .iter()
            .flat_map(|ir| ir.fields.values())
            .map(String::to_string)
            .collect();

        BrickDeclareField { name, variables }
    }
}

impl Display for BrickDeclareField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.variables.len() != 1 {
            write!(f, "        {}: [{}],", self.name, self.variables.join(", "))
        } else {
            write!(f, "        {}: {},", self.name, self.variables[0])
        }
    }
}
