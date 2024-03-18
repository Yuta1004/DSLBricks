use std::collections::HashMap;

use crate::xml::{BlocklyXML, BlocklyXMLBlock, BlocklyXMLBlockComponent, BlocklyXMLValue};

#[derive(Debug)]
pub struct BlocklyIR {
    pub r#type: String,
    pub fields: HashMap<String, String>,
    pub blocks: HashMap<String, Vec<BlocklyIR>>,
}

impl From<BlocklyXML> for Vec<BlocklyIR> {
    fn from(xml: BlocklyXML) -> Self {
        xml.values
            .into_iter()
            .filter_map(|value| match value {
                BlocklyXMLValue::Block(block) => Some(block.into()),
                _ => None,
            })
            .collect()
    }
}

impl From<BlocklyXMLBlock> for BlocklyIR {
    fn from(block: BlocklyXMLBlock) -> Self {
        let r#type = block.r#type;
        let components = block.components.into_iter().map(Into::into).collect();

        BlocklyIR::from((r#type, components))
    }
}

impl From<(String, Vec<BlocklyIRComponent>)> for BlocklyIR {
    fn from((r#type, components): (String, Vec<BlocklyIRComponent>)) -> Self {
        let (mut found_fields, mut found_blocks) = (HashMap::new(), HashMap::new());
        for component in components {
            match component {
                BlocklyIRComponent::Field { name, value } => {
                    found_fields.insert(name, value);
                }
                BlocklyIRComponent::Blocks { name, blocks } => {
                    found_blocks.insert(name, blocks);
                }
            };
        }

        BlocklyIR {
            r#type,
            fields: found_fields,
            blocks: found_blocks,
        }
    }
}

#[derive(Debug)]
enum BlocklyIRComponent {
    Field {
        name: String,
        value: String,
    },
    Blocks {
        name: String,
        blocks: Vec<BlocklyIR>,
    },
}

impl From<BlocklyXMLBlockComponent> for BlocklyIRComponent {
    fn from(component: BlocklyXMLBlockComponent) -> Self {
        match component {
            BlocklyXMLBlockComponent::Field(field) => {
                let name = field.name;
                let value = if let Some(value) = field.value {
                    value
                } else {
                    "".to_string()
                };
                BlocklyIRComponent::Field { name, value }
            }
            BlocklyXMLBlockComponent::Statement(statement) => {
                let name = statement.name;
                let blocks = expand_xml_block(statement.block);
                BlocklyIRComponent::Blocks { name, blocks }
            }
            _ => unreachable!(),
        }
    }
}

fn expand_xml_block(block: BlocklyXMLBlock) -> Vec<BlocklyIR> {
    let mut blocks = vec![];

    let mut now_target = Some(block);
    while let Some(target) = now_target {
        let (mut next_target, mut components) = (None, vec![]);
        for component in target.components {
            match component {
                BlocklyXMLBlockComponent::Next { block } => {
                    next_target = Some(block);
                }
                _ => components.push(BlocklyIRComponent::from(component)),
            }
        }
        blocks.push(BlocklyIR::from((target.r#type, components)));
        now_target = next_target;
    }

    blocks
}
