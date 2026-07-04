//! Background data sources for text-info: stdin (incremental chunks) and
//! file watching (poll + replace). Each source runs on its own thread and
//! wakes the UI with `request_repaint` after every event.

use std::io::Read;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

use eframe::egui;

pub enum SourceEvent {
    /// New stdin data arrived; append to the current content.
    Append(String),
    /// The watched file changed; replace the whole content.
    Replace(String),
    /// The stream ended (stdin EOF); closing now counts as a normal close.
    Finished,
}

pub fn spawn_stdin(ctx: egui::Context) -> Receiver<SourceEvent> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut stdin = std::io::stdin().lock();
        let mut buf = [0u8; 8192];
        // Bytes of a UTF-8 sequence split across reads.
        let mut pending: Vec<u8> = Vec::new();
        loop {
            match stdin.read(&mut buf) {
                Ok(0) | Err(_) => {
                    if !pending.is_empty() {
                        let tail = String::from_utf8_lossy(&pending).into_owned();
                        send(&tx, &ctx, SourceEvent::Append(tail));
                    }
                    send(&tx, &ctx, SourceEvent::Finished);
                    break;
                }
                Ok(n) => {
                    pending.extend_from_slice(&buf[..n]);
                    let valid_len = match std::str::from_utf8(&pending) {
                        Ok(_) => pending.len(),
                        Err(err) => err.valid_up_to(),
                    };
                    if valid_len > 0 {
                        let chunk = String::from_utf8_lossy(&pending[..valid_len]).into_owned();
                        pending.drain(..valid_len);
                        if !send(&tx, &ctx, SourceEvent::Append(chunk)) {
                            break;
                        }
                    }
                }
            }
        }
    });
    rx
}

pub fn spawn_watch(ctx: egui::Context, path: PathBuf, interval: Duration) -> Receiver<SourceEvent> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut last: Option<String> = None;
        loop {
            // A missing file is treated as "no content yet", like the AHK
            // poller this replaces.
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            if last.as_deref() != Some(content.as_str()) {
                last = Some(content.clone());
                if !send(&tx, &ctx, SourceEvent::Replace(content)) {
                    break;
                }
            }
            std::thread::sleep(interval);
        }
    });
    rx
}

fn send(tx: &Sender<SourceEvent>, ctx: &egui::Context, event: SourceEvent) -> bool {
    let ok = tx.send(event).is_ok();
    if ok {
        ctx.request_repaint();
    }
    ok
}
