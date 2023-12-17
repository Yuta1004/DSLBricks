import { useState, useEffect } from "react";

import AppBar from "@mui/material/AppBar";
import ToolBar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";

import Split from "react-split";

import Editor from "../components/Editor";
import SideView from "../components/SideView";

import { openProject, saveProject, exportProject, genRustCode } from "../tauri/Command";

export default function App() {
    const [paneResizedCnt, setPaneResizedCnt] = useState<number>(0);
    const [xml, setXml] = useState<string>("");
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
                    onClick={() => openProject(setXml)}
                >
                    Open
                </Button>
                <Button
                    color="inherit"
                    onClick={() => saveProject(xml)}
                >
                    Save
                </Button>
                <Button
                    color="inherit"
                    onClick={exportProject}
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
            onDrag={() => { setPaneResizedCnt((cnt) => cnt + 1) }}
        >
            <Editor
                noticeResize={paneResizedCnt}
                xml={xml}
                onUpdate={setXml}
            />
            <SideView
                xml={xml}
                rust={rust}
            />
        </Split>
    </>);
}
