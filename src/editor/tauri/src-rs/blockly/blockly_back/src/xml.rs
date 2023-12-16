use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlocklyXML {
    #[serde(rename = "$value")]
    pub values: Vec<BlocklyXMLValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BlocklyXMLValue {
    #[serde(rename = "variables")]
    Variables(BlocklyXMLVariable),
    #[serde(rename = "block")]
    Block(BlocklyXMLBlock),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlocklyXMLVariable {
    pub variable: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlocklyXMLBlock {
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(rename = "$value")]
    pub components: Vec<BlocklyXMLBlockComponent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BlocklyXMLBlockComponent {
    #[serde(rename = "field")]
    Field(BlocklyXMLField),
    #[serde(rename = "statement")]
    Statement(BlocklyXMLStatement),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlocklyXMLField {
    pub name: String,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlocklyXMLStatement {
    pub name: String,
    #[serde(rename = "block")]
    pub blocks: Vec<BlocklyXMLBlock>,
}

#[cfg(test)]
mod test {
    use super::BlocklyXML;

    #[test]
    fn raw_xml() {
        let target = r#"
<xml xmlns="https://developers.google.com/blockly/xml">
    <variables>
        <variable>var</variable>
    </variables>
    <block type="std.statement.c.If" id="id" x="0" y="0">
        <field name="variable" id="">var</field>
        <field name="property" />
        <statement name="cond">
            <block type="brick" id="">
                <field name="DSLBrick" id="">var</field>
            </block>
        </statement>
    </block>
</xml>
        "#;
        serde_xml_rs::from_str::<BlocklyXML>(target).unwrap();
    }
}
