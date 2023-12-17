use std::thread;
use std::process::{Command, Child};
use std::io::{Write, BufRead, BufReader};
use std::time::Duration;

use os_pipe::PipeWriter;
use tauri::InvokeError;

static mut PROCESS: Option<Child> = None;
static mut PROCESS_STDIN: Option<PipeWriter> = None;
static mut PROCESS_STDOUT: Option<Vec<String>> = None;

#[tauri::command]
pub fn create_verify_process() -> Result<(), InvokeError> {
    // Check PROCESS variable
    unsafe {
        if PROCESS.is_some() {
            return Err(InvokeError::from("Verification process is already launched."));
        }
    }

    // Launch a verification process
    let (stdin_process, stdin_us) = os_pipe::pipe().unwrap();
    let (stdout_us, stdout_process) = os_pipe::pipe().unwrap();
    let process = Command::new("bash")
        .stdin(stdin_process)
        .stdout(stdout_process.try_clone().unwrap())
        .stderr(stdout_process)
        .spawn()
        .unwrap();

    // Start thread to capture stdout
    thread::spawn(move || {
        for line in BufReader::new(stdout_us).lines() {
            if let Ok(line) = line {
                unsafe { PROCESS_STDOUT.as_mut().unwrap().push(line); }
            }
        }
    });

    // Update PROCESS variable
    unsafe {
        PROCESS = Some(process);
        PROCESS_STDIN = Some(stdin_us);
        PROCESS_STDOUT = Some(vec![]);
    }

    Ok(())
}

#[tauri::command]
pub fn connect_verify_process(msg: &str) -> Result<String, InvokeError> {
    // Attach to a verification process
    let (_, stdin) = unsafe {
        match &mut PROCESS {
            Some(process) => (process, PROCESS_STDIN.as_mut().unwrap()),
            None => return Err(InvokeError::from("Verification process is not launched."))
        }
    };

    // Send msg to verification process
    write!(stdin, "{}", msg).unwrap();
    thread::sleep(Duration::from_secs(1));

    // Receive msg from verification process
    let recv = unsafe {
        let recv = PROCESS_STDOUT.as_ref().unwrap().join("\n");
        PROCESS_STDOUT = Some(vec![]);
        recv
    };

    Ok(recv)
}

#[tauri::command]
pub fn finish_verify_process() -> Result<(), InvokeError> {
    // Attach to a verification process
    let (process, _) = unsafe {
        match &mut PROCESS {
            Some(process) => (process, PROCESS_STDIN.as_mut().unwrap()),
            None => return Err(InvokeError::from("Verification process is not launched."))
        }
    };

    // Kill the process
    process.kill().unwrap();

    // Reset variables
    unsafe {
        PROCESS = None;
        PROCESS_STDIN = None;
        PROCESS_STDOUT = None;
    }

    Ok(())
}
