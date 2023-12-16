import Box from "@mui/material/Box";

import XMLViewer from "react-xml-viewer";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";

type CodePanelProps = {
    xml: string,
    rust: string,
};

export default function CodePanel(props: CodePanelProps) {
    return (
        <Box>
            <h3>Rust</h3>
            <SyntaxHighlighter language="rust">
                { props.rust }
            </SyntaxHighlighter>
            <h3>XML</h3>
            <XMLViewer xml={props.xml} />
        </Box>
    );
}
