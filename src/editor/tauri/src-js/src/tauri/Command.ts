import { invoke } from "@tauri-apps/api/tauri";

export function openProject(callback: (xml: string) => void) {
    const wrappedCallback = (xml: string|null) => xml && callback(xml);
    const ipc = async () => {
        invoke<string | null>("open_project", {}).then(wrappedCallback);
    };
    ipc();
};

export function saveProject(xml: string) {
    const ipc = async () => {
        await invoke<void>("save_project", { xml });
    };
    ipc();
};

export function exportProject() {
    const ipc = async () => {
        await invoke<string>("export_project", {});
    };
    ipc();
};

export function genRustCode(xml: string, callback: (rust: string) => void) {
    const ipc = async () => {
        invoke<string>("genrs", { xml }).then(callback);
    };
    ipc();
};
