import { invoke } from "@tauri-apps/api/tauri";

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

export async function exportProject(callback: () => void) {
    const ipc = async () => {
        invoke<string>("export_project", {}).then(callback);
    };
    ipc();
};

export function genRustCode(xml: string, callback: (rust: string) => void) {
    const ipc = async () => {
        invoke<string>("genrs", { xml }).then(callback);
    };
    ipc();
};
