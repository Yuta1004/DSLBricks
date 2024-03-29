import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

export function openProject(callback: (xml: string) => void) {
    const wrappedCallback = (xml: string|null) => xml && callback(xml);
    const ipc = async () => {
        invoke<string | null>("open_project", {}).then(wrappedCallback);
    };
    ipc();
};

export async function saveProject(xml: string, callback: () => void) {
    const ipc = async () => {
        invoke<void>("save_project", { xml }).then(callback);
    };
    ipc();
};

export async function exportProject(xml: string, rust: string, callback: () => void) {
    const ipc = async () => {
        invoke<string>("export_project", { xml, rust }).then(callback);
    };
    ipc();
};

export function genRustCode(xml: string, callback: (rust: string) => void) {
    const ipc = async () => {
        invoke<string>("genrs", { xml }).then(callback);
    };
    ipc();
};

export function createSubprocess(stdout_handler: (line: string) => void, callback: () => void) {
    const ipc = async () => {
        listen("subprocess_stdout", event => stdout_handler(event.payload));
        invoke<void>("create_subprocess", {}).then(callback);
    };
    ipc();
}

export function connectSubprocess(msg: string, callback: () => void) {
    const ipc = async () => {
        invoke<void>("connect_subprocess", { msg }).then(callback);
    };
    ipc();
}

export function finishSubprocess(callback: () => void) {
    const ipc = async () => {
        invoke<void>("finish_subprocess", {}).then(callback);
    };
    ipc();
}
