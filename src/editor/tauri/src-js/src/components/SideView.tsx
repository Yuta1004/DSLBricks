import { useState } from "react";

import Box from "@mui/material/Box";
import Tabs from "@mui/material/Tabs";
import Tab from "@mui/material/Tab";

import CodePanel from "./CodePanel";
import StatusPanel from "./StatusPanel";
import CatalogPanel from "./CatalogPanel";
import LogPanel from "./LogPanel";

type SideViewProps = {
    xml: string,
    rust: string,
};

export default function SideView(props: SideViewProps) {
    const [tabStat, setTabStat] = useState<number>(0);

    return (
        <Box>
            <Tabs
                value={tabStat}
                onChange={(_, newStat) => setTabStat(newStat)}
                centered
            >
                <Tab label="Code" />
                <Tab label="Status" />
                <Tab label="Catalog" />
                <Tab label="Log" />
            </Tabs>
            <div style={{ margin: "8px" }}>
                { tabStat === 0 &&
                    <CodePanel
                        xml={props.xml}
                        rust={props.rust}
                    />
                }
                { tabStat === 1 &&
                    <StatusPanel />
                }
                { tabStat === 2 &&
                    <CatalogPanel />
                }
                { tabStat === 3 &&
                    <LogPanel log="Ok" />
                }
            </div>
        </Box>
    );
}
