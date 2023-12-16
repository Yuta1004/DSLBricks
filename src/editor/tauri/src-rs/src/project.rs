use serde::{Serialize, Deserialize};

use blockly::back::xml::BlocklyXML;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    blockly: BlocklyXML,
}
