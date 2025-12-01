use std::sync::mpsc::Receiver;

use crate::client;

pub fn receive_tui_message(rx: &Option<Receiver<client::TuiMessage>>) -> Option<client::TuiMessage> {
    if let Some(rx) = rx {
        match rx.try_recv() {
            Ok(msg) => Some(msg),
            Err(_) => None,
        }
    } else {
        None
    }
}

pub fn send_tui_message(
    message: client::TuiMessage,
    tx: &Option<std::sync::mpsc::Sender<client::TuiMessage>>,
) {
    if let Some(tx) = tx {
        let _ = tx.send(message);
    }
}
