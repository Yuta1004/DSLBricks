use std::thread;
use std::process::{Command, Child};
use std::io::{Write, BufRead, BufReader};
use std::time::Duration;
use std::sync::{Arc, Mutex};

use os_pipe::PipeWriter;
use tauri::InvokeError;

static mut SUBPROCESS: Option<Arc<Mutex<Subprocess>>> = None;

#[tauri::command]
pub fn create_verify_process() -> Result<(), InvokeError> {
    unsafe {
        if SUBPROCESS.is_some() {
            return Err(InvokeError::from("Verification process is already launched."));
        }
    }

    match Subprocess::new("bash") {
        Ok(subprocess) => {
            unsafe { SUBPROCESS = Some(subprocess); }
            Ok(())
        }
        Err(err) => Err(InvokeError::from(format!("{}", err)))
    }
}

#[tauri::command]
pub fn connect_verify_process(msg: &str) -> Result<String, InvokeError> {
    let subprocess = unsafe {
        match &SUBPROCESS {
            Some(subprocess) => subprocess,
            None => return Err(InvokeError::from("Verification process is not launched."))
        }
    };

    Subprocess::stdin(Arc::clone(subprocess), msg).unwrap();
    thread::sleep(Duration::from_millis(500));
    let stdout = Subprocess::stdout(Arc::clone(subprocess)).unwrap();

    Ok(stdout)
}

#[tauri::command]
pub fn finish_verify_process() -> Result<(), InvokeError> {
    let subprocess = unsafe {
        match &SUBPROCESS {
            Some(subprocess) => subprocess,
            None => return Err(InvokeError::from("Verification process is not launched."))
        }
    };

    Subprocess::kill(Arc::clone(subprocess)).unwrap();
    unsafe { SUBPROCESS = None; }

    Ok(())
}

struct Subprocess {
    process: Child,
    stdin: PipeWriter,
    stdout: Vec<String>,
}

impl Subprocess {
    pub fn new(cmd: &str) -> anyhow::Result<Arc<Mutex<Self>>> {
        let (stdin_process, stdin_us) = os_pipe::pipe()?;
        let (stdout_us, stdout_process) = os_pipe::pipe()?;
        let process = Command::new(cmd)
            .stdin(stdin_process)
            .stdout(stdout_process.try_clone().unwrap())
            .stderr(stdout_process)
            .spawn()?;

        // Init myself
        let subprocess = Subprocess {
            process,
            stdin: stdin_us,
            stdout: vec![],
        };
        let subprocess = Arc::new(Mutex::new(subprocess));

        // Start thread to capture stdout
        let subprcess_in_thread = Arc::clone(&subprocess);
        thread::spawn(move || {
            for line in BufReader::new(stdout_us).lines() {
                if let Ok(line) = line {
                    subprcess_in_thread
                        .lock()
                        .unwrap()
                        .stdout
                        .push(line);
                }
            }
        });

        Ok(subprocess)
    }

    pub fn stdin(subprocess: Arc<Mutex<Self>>, input: &str) -> anyhow::Result<()> {
        let mut stdin = &subprocess.lock().unwrap().stdin;
        write!(&mut stdin, "{}", input)?;
        Ok(())
    }

    pub fn stdout(subprocess: Arc<Mutex<Self>>) -> anyhow::Result<String> {
        let stdout = subprocess.lock().unwrap().stdout.join("\n");
        *(&mut subprocess.lock().unwrap().stdout) = vec![];
        Ok(stdout)
    }

    pub fn kill(subprocess: Arc<Mutex<Self>>) -> anyhow::Result<()> {
        subprocess.lock().unwrap().process.kill()?;
        Ok(())
    }
}
