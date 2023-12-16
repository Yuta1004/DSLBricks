import { useState, useEffect } from "react";

import AppBar from "@mui/material/AppBar";
import ToolBar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";

import Split from "react-split";

import { invoke } from "@tauri-apps/api/tauri";

import Editor from "../components/Editor";
import SideView from "../components/SideView";

export default function App() {
    const [paneResizedCnt, setPaneResizedCnt] = useState<number>(0);
    const [xml, setXml] = useState<string>("");
    const [rust, setRust] = useState<string>("fn main() { }");

    useEffect(() => {
        (async() => {
            invoke<string>("genrs", { xml })
                .then(rust => setRust(rust))
                .catch(console.error);
        })()
    }, [xml]);

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
                <Button color="inherit">Open</Button>
                <Button color="inherit">Save</Button>
                <Button color="inherit">Export</Button>
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
                onUpdate={setXml}
            />
            <SideView
                xml={xml}
                rust={rust}
            />
        </Split>
    </>);
}
