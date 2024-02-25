pub mod ir;
mod js;

use std::fs::File;
use std::fmt::Write as FWrite;
use std::io::Write as IWrite;

pub fn gen_ts_files(dir: &str, toolbox: ir::ToolBox) -> anyhow::Result<()> {
    gen_ts_file_blocks(dir.to_string() + "/blocks.ts", &toolbox)?;
    gen_ts_file_toolbox(dir.to_string() + "/toolbox.ts", &toolbox)?;
    Ok(())
}

fn gen_ts_file_blocks(path: String, toolbox: &ir::ToolBox) -> anyhow::Result<()> {
    // IR to Blocks(js)
    fn expand_toolbox(f: &mut String, toolbox: &ir::ToolBox) {
        for item in &toolbox.items {
            match item {
                ir::ToolBoxItem::Block(block) => writeln!(f, "{}", js::Block::from(block)).unwrap(),
                ir::ToolBoxItem::ToolBox(toolbox) => expand_toolbox(f, toolbox),
            }
        }
    }
    let mut blocks_js = String::new();
    expand_toolbox(&mut blocks_js, toolbox);

    // Write
    let mut f = File::create(path)?;
    writeln!(&mut f, r#"
        // Note: This is auto generated file.

        import Blockly from "blockly";

        {}
    "#, blocks_js)?;

    Ok(())
}

fn gen_ts_file_toolbox(path: String, toolbox: &ir::ToolBox) -> anyhow::Result<()> {
    // IR to ToolBox(js)
    let toolbox_js = js::ToolBox::from(toolbox);

    // Write
    let mut f = File::create(path)?;
    writeln!(&mut f, r#"
        // Note: This is auto generated file.

        const ToolBox = {};

        export default ToolBox;
    "#, toolbox_js)?;

    Ok(())
}
