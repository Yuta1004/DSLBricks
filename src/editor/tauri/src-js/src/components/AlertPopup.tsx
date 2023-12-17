import { useState, useEffect } from "react";

import Alert, { AlertColor } from "@mui/material/Alert";
import AlertTitle from "@mui/material/AlertTitle";

type AlertPopupProps = {
    posX: string,
    posY: string,
    kind: AlertColor,
    message: string,
}

export default function AlertPopup(props: AlertPopupProps) {
    const toTitle = (s: string) => {
        return s.charAt(0).toUpperCase() + s.substring(1);
    };

    const [visible, setVisible] = useState<boolean>(false);

    useEffect(() => {
        setVisible(props.message.length > 0);
        setTimeout(() => setVisible(false), 8000);
    }, [props.message])

    return (
        <div
            style={{
                position: "absolute",
                left: props.posX,
                top: props.posY,
                zIndex: 9999,
                visibility: (visible ? "visible" : "hidden"),
            }}
        >
            <Alert
                severity={props.kind}
                onClose={() => setVisible(false) }
            >
                <AlertTitle>{toTitle(props.kind)}</AlertTitle>
                {props.message}
            </Alert>
        </div>
    );
}
