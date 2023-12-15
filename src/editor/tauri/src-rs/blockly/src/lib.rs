pub mod ir;
pub mod block;
pub mod toolbox;

use std::fs::File;
use std::collections::HashMap;
use std::io::Write;

use ir::BlocklyIR;
use block::BlocklyBlock;
use toolbox::BlocklyToolBox;

pub fn gen_ts_files(dir: &str, ir_set: HashMap<&str, Vec<BlocklyIR>>) -> anyhow::Result<()> {
    gen_ts_file_blocks(dir.to_string() + "/blocks.ts", &ir_set)?;
    gen_ts_file_toolbox(dir.to_string() + "/toolbox.ts", &ir_set)?;
    Ok(())
}

fn gen_ts_file_blocks(path: String, ir_set: &HashMap<&str, Vec<BlocklyIR>>) -> anyhow::Result<()> {
    // IR(s) to Block
    let mut blocks = vec![];
    for (_, irs) in ir_set {
        for ir in irs {
            blocks.push(BlocklyBlock::from(ir));
        }
    }

    // Write
    let mut f = File::create(path)?;
    writeln!(&mut f, r#"
        // Note: This is auto generated file.

        import Blockly from "blockly";

        Blockly.Blocks["brick"] = {{
            init: function() {{
                this.jsonInit({{
                    "type": "brick",
                    "message0": "DSLBrick %1",
                    "args0": [
                        {{
                            "type": "field_variable",
                            "name": "NAME",
                            "variable": "item"
                        }}
                    ],
                    "inputsInline": false,
                    "previousStatement": null,
                    "nextStatement": null,
                    "colour": 230,
                    "tooltip": "",
                    "helpUrl": ""
                }})
            }}
        }}
    "#)?;
    for block in blocks {
        writeln!(&mut f, "{}", block)?;
    }

    Ok(())
}

fn gen_ts_file_toolbox(path: String, ir_set: &HashMap<&str, Vec<BlocklyIR>>) -> anyhow::Result<()> {
    // IR(s) to ToolBox
    let mut toolboxes = vec![];
    for (name, irs) in ir_set {
        toolboxes.push(BlocklyToolBox::from((*name, irs.as_slice())));
    }

    // Write
    let mut f = File::create(path)?;
    writeln!(&mut f, r#"
        // Note: This is auto generated file.

        const ToolBox = {{
            kind: "categoryToolbox",
            contents: [
                {{
                    kind: "category",
                    name: "Default",
                    contents: [
                        {{
                            kind: "block",
                            type: "brick",
                        }}
                    ]
                }},
    "#)?;
    for toolbox in toolboxes {
        writeln!(&mut f, "{}", toolbox)?;
    }
    writeln!(&mut f, r#"
            ]
        }};
        export default ToolBox;
    "#)?;

    Ok(())
}
