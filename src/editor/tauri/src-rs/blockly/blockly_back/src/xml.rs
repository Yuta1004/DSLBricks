use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlocklyXML {
    #[serde(rename = "$value")]
    pub values: Vec<BlocklyXMLValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlocklyXMLValue {
    Variables(BlocklyXMLVariables),
    Block(BlocklyXMLBlock),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlocklyXMLVariables {
    #[serde(rename = "$value")]
    pub variable: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlocklyXMLBlock {
    pub id: String,
    pub r#type: String,
    pub x: Option<i32>,
    pub y: Option<i32>,
    #[serde(rename = "$value")]
    pub components: Vec<BlocklyXMLBlockComponent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlocklyXMLBlockComponent {
    Field(BlocklyXMLField),
    Statement(BlocklyXMLStatement),
    Next { block: BlocklyXMLBlock },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlocklyXMLField {
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlocklyXMLStatement {
    pub name: String,
    #[serde(rename = "$value")]
    pub block: BlocklyXMLBlock,
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
        <variable>var</variable>
    </variables>
    <block type="std.statement.c.If" id="id" x="0" y="0">
        <field name="variable" id="">var</field>
        <field name="property" />
        <statement name="cond">
            <block type="brick" id="">
                <field name="DSLBrick" id="">var</field>
                <next>
                    <block type="brick" id="">
                        <field name="DSLBrick" id="">var</field>
                    </block>
                    <next>
                        <block type="brick" id="">
                            <field name="DSLBrick" id="">var</field>
                        </block>
                    </next>
                </next>
            </block>
        </statement>
    </block>
</xml>
        "#;
        serde_xml_rs::from_str::<BlocklyXML>(target).unwrap();
    }
}
