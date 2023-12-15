import Box from "@mui/material/Box";

type LogPanelProps = {
    log: string
}

export default function LogPanel(props: LogPanelProps) {
    return (
        <Box>
            <textarea
                style={{
                    width: "100%",
                    height: "calc(100vh - 144px)",
                }}
                readOnly
            >
                { props.log }
            </textarea>
        </Box>
    );
}
