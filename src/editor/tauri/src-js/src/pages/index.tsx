import { useState } from "react";

import AppBar from "@mui/material/AppBar";
import ToolBar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";

import Split from "react-split";

import Editor from "./Editor";
import SideView from "./SideView";

export default function App() {
    const [paneResizedCnt, setPaneResizedCnt] = useState<number>(0);
    const [xml, setXml] = useState<string>("");

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
                rust="fn main() { }"
            />
        </Split>
    </>);
}
