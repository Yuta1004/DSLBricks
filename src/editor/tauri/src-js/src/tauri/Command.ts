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

export function createVerifyProcess(callback: () => void) {
    const ipc = async () => {
        invoke<void>("create_verify_process", {}).then(callback);
    };
    ipc();
}

export function connectVerifyProcess(msg: string, callback: (recv: string) => void) {
    const ipc = async () => {
        invoke<string>("connect_verify_process", { msg }).then(callback);
    };
    ipc();
}

export function finishVerifyProcess(callback: () => void) {
    const ipc = async () => {
        invoke<void>("finish_verify_process", {}).then(callback);
    };
    ipc();
}
