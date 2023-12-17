import { useState, useEffect } from "react";

import Blockly from "blockly";
import { BlocklyWorkspace, WorkspaceSvg } from "react-blockly";

import ToolBox from "../custom/toolbox";
import "../custom/blocks";

type EditorProps = {
    noticeResize: number,
    noticeResetXML: number,
    xml: string,
    onUpdate: (xml: string) => void,
}

export default function Editor(props: EditorProps) {
    var [ws, setWs] = useState<WorkspaceSvg>(null);

    useEffect(() => {
        if (ws) {
            const xml = document.createElement("div");
            xml.innerHTML = props.xml;
            Blockly.Xml.clearWorkspaceAndLoadFromXml(xml.firstElementChild, ws);
        }
    }, [props.noticeResetXML]);

    useEffect(() => {
        if (ws) {
            Blockly.svgResize(ws);
        }
    }, [props.noticeResize]);

    return (
        <BlocklyWorkspace
            className="workspace"
            workspaceConfiguration={{
                grid: {
                    spacing: 50,
                    length: 3
                },
                zoom: {
                    controls: true,
                    wheel: true
                }
            }}
            toolboxConfiguration={ToolBox}
            onXmlChange={props.onUpdate}
            onWorkspaceChange={setWs}
        />
    );
}
