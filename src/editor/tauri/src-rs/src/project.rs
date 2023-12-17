use serde::{Serialize, Deserialize};

use blockly::back::xml::BlocklyXML;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub blockly: BlocklyXML,
}

impl Project {
    pub fn new(blockly: BlocklyXML) -> Self {
        Project { blockly }
    }
}
