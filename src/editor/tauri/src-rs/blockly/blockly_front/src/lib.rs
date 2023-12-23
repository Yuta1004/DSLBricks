pub mod ir;
pub mod json;

use std::fs::File;
use std::fmt::Write as FWrite;
use std::io::Write as IWrite;

use json::block::Block;
use json::toolbox::ToolBox;

pub fn gen_ts_files(dir: &str, ir_set: &[(&str, Vec<ir::Block>)]) -> anyhow::Result<()> {
    gen_ts_file_blocks(dir.to_string() + "/blocks.ts", &ir_set)?;
    gen_ts_file_toolbox(dir.to_string() + "/toolbox.ts", &ir_set)?;
    Ok(())
}

fn gen_ts_file_blocks(path: String, ir_set: &[(&str, Vec<ir::Block>)]) -> anyhow::Result<()> {
    // IR(s) to Block
    let mut blocks = String::new();
    for (_, irs) in ir_set {
        for ir in irs {
            writeln!(&mut blocks, "{}", Block::from(ir))?;
        }
    }

    // Write
    let mut f = File::create(path)?;
    writeln!(&mut f, r#"
        // Note: This is auto generated file.

        import Blockly from "blockly";

        {}
    "#, blocks)?;

    Ok(())
}

fn gen_ts_file_toolbox(path: String, ir_set: &[(&str, Vec<ir::Block>)]) -> anyhow::Result<()> {
    // IR(s) to ToolBox
    let mut toolboxes = String::new();
    for (name, irs) in ir_set {
        writeln!(&mut toolboxes, "{},", ToolBox::from((*name, irs.as_slice())))?;
    }

    // Write
    let mut f = File::create(path)?;
    writeln!(&mut f, r#"
        // Note: This is auto generated file.

        const ToolBox = {{
            kind: "categoryToolbox",
            contents: [{}]
        }};
        export default ToolBox;
    "#, toolboxes)?;

    Ok(())
}
