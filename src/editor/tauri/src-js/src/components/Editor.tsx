import { useState, useEffect } from "react";

import Blockly from "blockly";
import { BlocklyWorkspace, WorkspaceSvg } from "react-blockly";

import ToolBox from "../custom/toolbox";
import "../custom/blocks";

type EditorProps = {
    noticeResize: number,
    xml: string,
    onUpdate: (xml: string) => void,
}

export default function Editor(props: EditorProps) {
    var ws: WorkspaceSvg = null;

    const [status, setStatus] = useState<string>("Initialized");

    useEffect(() => {
        if (ws) {
            Blockly.svgResize(ws);
        }
    }, [props.noticeResize]);

    useEffect(() => {
        if (ws) {
            const xml = document.createElement("div");
            xml.innerHTML = props.xml;
            Blockly.Xml.clearWorkspaceAndLoadFromXml(xml.firstElementChild, ws);
        }
    }, [props.xml]);

    return (
        <div className="workspace-outer">
            <BlocklyWorkspace
                className="workspace"
                toolboxConfiguration={ToolBox}
                initialXml={props.xml}
                onXmlChange={props.onUpdate}
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
