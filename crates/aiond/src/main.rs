use aion_core::defaults::{DefaultPolicy, EchoBackend, LogMediator};
use aion_core::{Intent, Pipeline, Registry};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

fn socket_path() -> PathBuf {
    std::env::var_os("AION_SOCKET")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/run/aion/aion.sock"))
}

fn err_json(msg: &str) -> String {
    serde_json::json!({ "error": msg }).to_string()
}

/// One JSON intent per line in, one JSON outcome per line out.
fn serve(stream: UnixStream, pipeline: Arc<Pipeline>) {
    let mut writer = match stream.try_clone() {
        Ok(w) => w,
        Err(_) => return,
    };
    for line in BufReader::new(stream).lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        let reply = match serde_json::from_str::<Intent>(&line) {
            Ok(intent) => match pipeline.handle(&intent, None) {
                Ok(outcome) => serde_json::to_string(&outcome)
                    .unwrap_or_else(|e| err_json(&e.to_string())),
                Err(e) => err_json(&e),
            },
            Err(e) => err_json(&format!("bad intent: {e}")),
        };
        if writeln!(writer, "{reply}").is_err() {
            break;
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut registry = Registry::default();
    registry.register(Box::new(EchoBackend));
    let pipeline = Arc::new(Pipeline {
        registry,
        policy: Box::new(DefaultPolicy),
        mediator: Box::new(LogMediator),
    });

    let path = socket_path();
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let _ = std::fs::remove_file(&path);
    let listener = UnixListener::bind(&path)?;
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
    eprintln!("aiond listening on {}", path.display());

    for conn in listener.incoming() {
        match conn {
            Ok(stream) => {
                let p = Arc::clone(&pipeline);
                thread::spawn(move || serve(stream, p));
            }
            Err(e) => eprintln!("accept error: {e}"),
        }
    }
    Ok(())
}
