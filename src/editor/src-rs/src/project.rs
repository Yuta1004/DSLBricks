use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub blockly_xml: String, // BlocklyXML
}

impl Project {
    pub fn new(blockly_xml: String) -> Self {
        Project { blockly_xml }
    }
}
