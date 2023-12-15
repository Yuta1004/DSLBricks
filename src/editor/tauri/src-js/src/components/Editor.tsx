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

    const [xml, setXml] = useState<string>("<xml xmlns='https://developers.google.com/blockly/xml'/>");
    const [status, setStatus] = useState<string>("Initialized");

    useEffect(() => {
        if (ws) {
            Blockly.svgResize(ws);
        }
    }, [props.noticeResize]);

    return (
        <div className="workspace-outer">
            <BlocklyWorkspace
                className="workspace"
                toolboxConfiguration={ToolBox}
                initialXml={xml}
                onXmlChange={(xml) => {
                    setXml(xml);
                    props.onUpdate(xml);
                }}
                onWorkspaceChange={(newWs) => { ws = newWs; }}
            />
            <p
                style={{
                    width: "100%",
                    height: "14px",
                    margin: "0px",
                    fontSize: "14px",
                }}
            >
                {status}
            </p>
        </div>
    );
}
