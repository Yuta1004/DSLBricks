import { useState, useEffect } from "react";

import Blockly from "blockly";
import { BlocklyWorkspace, WorkspaceSvg } from "react-blockly";

import ToolBox from "../custom/toolbox";
import "../custom/bricks";

type EditorProps = {
    noticeUpdate: number,
}

export default function Editor(props: EditorProps) {
    var ws: WorkspaceSvg = null;

    const [xml, setXml] = useState<string>("");

    useEffect(() => {
        if (ws) {
            Blockly.svgResize(ws);
        }
    }, [props.noticeUpdate]);

    return (
        <BlocklyWorkspace
            className="fullscreen"
            toolboxConfiguration={ToolBox}
            initialXml={xml}
            onXmlChange={setXml}
            onWorkspaceChange={(newWs) => { ws = newWs; }}
        />
    );
}
