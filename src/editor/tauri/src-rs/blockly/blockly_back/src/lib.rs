pub mod xml;
pub mod ir;

use xml::BlocklyXML;
use ir::BlocklyIR;

pub fn parse_xml(xml: &str) -> anyhow::Result<Vec<BlocklyIR>> {
    let xml = serde_xml_rs::from_str::<BlocklyXML>(xml)?;
    Ok(xml.into())
}
