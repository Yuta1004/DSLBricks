import { useState } from "react";

import { BlocklyWorkspace } from "react-blockly";

export default function App() {
    const [xml, setXml] = useState("");

    return (
        <BlocklyWorkspace
            initialXml={xml}
            onXmlChange={setXml}
        />
    );
}
