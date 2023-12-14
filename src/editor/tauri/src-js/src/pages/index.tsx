import { useState } from "react";

import { BlocklyWorkspace } from "react-blockly";

import ToolBox from "../custom/toolbox";
import "../custom/bricks";

export default function App() {
    const [xml, setXml] = useState("");

    return (
        <BlocklyWorkspace
            className="fullscreen"
            toolboxConfiguration={ToolBox}
            initialXml={xml}
            onXmlChange={setXml}
        />
    );
}
