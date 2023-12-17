import { invoke } from "@tauri-apps/api/tauri";

export function openProject() {
    const ipc = async () => {
        await invoke<string>("open_project", {});
    };
    ipc();
};

export function saveProject() {
    const ipc = async () => {
        await invoke<string>("save_project", {});
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
