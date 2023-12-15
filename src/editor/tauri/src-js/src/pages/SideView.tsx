import { useState } from "react";

import Box from "@mui/material/Box";
import Tabs from "@mui/material/Tabs";
import Tab from "@mui/material/Tab";

export default function SideView() {
    const [tabStat, setTabStat] = useState<number>(0);

    return (
        <Box>
            <Tabs
                value={tabStat}
                onChange={(_, newStat) => setTabStat(newStat)}
                centered
            >
                <Tab label="Code"/>
                <Tab label="Status"/>
                <Tab label="Log"/>
            </Tabs>
            { tabStat === 0 && <CodePanel /> }
            { tabStat === 1 && <StatusPanel /> }
            { tabStat === 2 && <LogPanel /> }
        </Box>
    );
}

function CodePanel() {
    return (
        <Box>
            <h1>Code</h1>
        </Box>
    );
}

function StatusPanel() {
    return (
        <Box>
            <h1>Status</h1>
        </Box>
    );
}

function LogPanel() {
    return (
        <Box>
            <h1>Log</h1>
        </Box>
    );
}
