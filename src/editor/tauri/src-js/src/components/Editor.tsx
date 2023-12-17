import { useEffect } from "react";

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
        <BlocklyWorkspace
            className="workspace"
            toolboxConfiguration={ToolBox}
            initialXml={props.xml}
            onXmlChange={props.onUpdate}
            onWorkspaceChange={(newWs) => { ws = newWs; }}
        />
    );
}
