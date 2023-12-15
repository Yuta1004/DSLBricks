import { useState, useEffect } from "react";

import Blockly from "blockly";
import { BlocklyWorkspace, WorkspaceSvg } from "react-blockly";

import ToolBox from "../custom/toolbox";
import "../custom/bricks";

type EditorProps = {
    noticeResize: number,
    onUpdate: (xml: string) => void,
}

export default function Editor(props: EditorProps) {
    var ws: WorkspaceSvg = null;

    const [xml, setXml] = useState<string>("");

    useEffect(() => {
        if (ws) {
            Blockly.svgResize(ws);
        }
    }, [props.noticeResize]);

    return (
        <BlocklyWorkspace
            className="fullscreen"
            toolboxConfiguration={ToolBox}
            initialXml={xml}
            onXmlChange={(xml) => {
                setXml(xml);
                props.onUpdate(xml);
            }}
            onWorkspaceChange={(newWs) => { ws = newWs; }}
        />
    );
}
