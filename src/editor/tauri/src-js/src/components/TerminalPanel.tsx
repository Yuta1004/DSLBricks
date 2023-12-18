import { useState } from "react";

import Box from "@mui/material/Box";
import Switch from "@mui/material/Switch";

import Terminal, { ColorMode, TerminalOutput } from "react-terminal-ui";

import {
    createSubprocess,
    connectSubprocess,
    finishSubprocess
} from "../tauri/Command";

const Dark = ColorMode.Dark;
const Light = ColorMode.Light;

type TerminalPanelProps = {}

export default function TerminalPanel(props: TerminalPanelProps) {
    // States
    const [running, setRunning] = useState<boolean>(false);
    const [color, setColor] = useState<ColorMode>(ColorMode.Light);
    const [prompt, setPrompt] = useState<string>(">>");
    const [lines, setLines] = useState<JSX.Element[]>([
        <TerminalOutput>Welcome to Terminal.</TerminalOutput>,
        <TerminalOutput>Type 'help' to show available commmands.</TerminalOutput>
    ]);

    // Commands (for terminal)
    const compileCommand = (input: string) => {
        setLines(lines => {
            return [
                ...lines,
                <TerminalOutput>{">> " + input}</TerminalOutput>,
                <TerminalOutput>Compiling...</TerminalOutput>,
            ];
        });

        createSubprocess(
            (line) => {
                setLines(lines => {
                    return [
                        ...lines,
                        <TerminalOutput>{line}</TerminalOutput>,
                    ];
                });
            },
            () => {
                setLines(lines => {
                    return [
                        ...lines,
                        <TerminalOutput>Ok</TerminalOutput>,
                        <TerminalOutput/>
                    ];
                });
                setPrompt("$");
                setRunning(true);
            }
        );
    };

    const subprocessCommand = (input: string) => {
        if (input === "exit") {
            finishSubprocess(() => {
                setLines(lines => {
                    return [
                        ...lines,
                        <TerminalOutput>{ "$ exit" }</TerminalOutput>,
                        <TerminalOutput>bye...</TerminalOutput>,
                        <TerminalOutput/>
                    ];
                });
                setPrompt(">>");
                setRunning(false);
            });
            return;
        }

        setLines(lines => {
            return [
                ...lines,
                <TerminalOutput>{"$ " + input}</TerminalOutput>,
            ];
        });

        connectSubprocess(input+"\n", () => {});
    };

    const clearCommand = (input: string) => {
        setLines([]);
    };

    const helpCommand = (input: string) => {
        setLines(lines => {
            return [
                ...lines,
                <TerminalOutput>{">> " + input}</TerminalOutput>,
                <TerminalOutput>Available commands</TerminalOutput>,
                <TerminalOutput>  - 'compile' : Compile the DSL bricks.</TerminalOutput>,
                <TerminalOutput>  - 'clear' : Clear the input history.</TerminalOutput>,
                <TerminalOutput>  - 'help' : Show this message.</TerminalOutput>,
                <TerminalOutput/>
            ];
        });
    };

    const unknownCommand = (input: string) => {
        setLines(lines => {
            return [
                ...lines,
                <TerminalOutput>{">> " + input}</TerminalOutput>,
                <TerminalOutput>Command not registered.</TerminalOutput>,
                <TerminalOutput/>
            ];
        });
    };

    const evalInput = (input: string) => {
        if (running)             return subprocessCommand(input);
        if (input === "compile") return compileCommand(input);
        if (input === "clear")   return clearCommand(input);
        if (input === "help")    return helpCommand(input);
        else                     return unknownCommand(input);
    };

    // Setup terminal scroll
    (() => {
        if (typeof document !== "undefined") {
            const terminal = document.getElementsByClassName("react-terminal")[0] as HTMLDivElement;
            if (terminal != null) {
                const observer = new MutationObserver(() => {
                    terminal.scroll(0, terminal.scrollHeight - terminal.clientHeight);
                });
                observer.observe(terminal, { childList: true });
            }
        }
    })();

    return (
        <Box style={{ height: "100%" }}>
            <div
                style={{
                    position: "absolute",
                    right: "8px",
                    zIndex: 9999,
                }}
            >
                <Switch onChange={(event) => setColor(event.target.checked ? Dark : Light)}/>
            </div>
            <Terminal
                height="100%"
                colorMode={color}
                prompt={prompt}
                onInput={prompt ? evalInput : null}
            >
                { lines }
            </Terminal>
        </Box>
    );
}
