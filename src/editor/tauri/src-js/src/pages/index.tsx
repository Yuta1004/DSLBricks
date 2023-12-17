import { useState, useEffect } from "react";

import AppBar from "@mui/material/AppBar";
import ToolBar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import { AlertColor } from "@mui/material/Alert";

import Split from "react-split";

import Editor from "../components/Editor";
import SideView from "../components/SideView";
import AlertPopup from "../components/AlertPopup";

import {
    openProject,
    saveProject,
    exportProject,
    genRustCode,
} from "../tauri/Command";

export default function App() {
    const [status, setStatus] = useState<[AlertColor, string]>(["success", ""]);
    const [resizeCnt, setResizeCnt] = useState<number>(0);
    const [resetCnt, setResetCnt] = useState<number>(0);
    const [xml, setXml] = useState<string>('<xml xmlns="https://developers.google.com/blockly/xml" />');
    const [rust, setRust] = useState<string>("fn main() { }");

    useEffect(() => genRustCode(xml, setRust), [xml]);

    return (<>
        <AppBar position="static">
            <ToolBar>
                <Typography
                    variant="h5"
                    component="div"
                    sx={{ flexGrow: 1 }}
                >
                    DSL Bricks Editor
                </Typography>
                <Button
                    color="inherit"
                    onClick={() => openProject((xml) => {
                        setXml(xml);
                        setResetCnt(cnt => cnt + 1);
                        setStatus(["success", "The project has been successfully opened."]);
                    })}
                >
                    Open
                </Button>
                <Button
                    color="inherit"
                    onClick={() => saveProject(xml, () => {
                        setStatus(["success", "The project has been successfully saved."]);
                    })}
                >
                    Save
                </Button>
                <Button
                    color="inherit"
                    onClick={() => exportProject(xml, rust, () => {
                        setStatus(["success", "The project has been successfully exported."]);
                    })}
                >
                    Export
                </Button>
            </ToolBar>
        </AppBar>
        <Split
            className="split-flex"
            gutterSize={5}
            sizes={[70, 30]}
            direction="horizontal"
            onDrag={() => setResizeCnt(cnt => cnt + 1)}
        >
            <Editor
                noticeResize={resizeCnt}
                noticeResetXML={resetCnt}
                xml={xml}
                onUpdate={setXml}
            />
            <SideView
                xml={xml}
                rust={rust}
            />
        </Split>
        <AlertPopup
            posX="10px"
            posY="calc(100vh - 92px)"
            kind={status[0]}
            message={status[1]}
        />
    </>);
}
