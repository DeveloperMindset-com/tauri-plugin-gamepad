use tauri::{Emitter, EventTarget, Manager};

use gilrs::{Event, EventType, Gamepad, Gilrs, MappingSource};

use serde_json::{json, Value};

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{SystemTime, UNIX_EPOCH};

use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    AppHandle, Runtime, Window,
};

mod utils;
use crate::utils::{axis_from_u16, button_from_u16};

/// Shared plugin state. Consuming apps can read/write the logging flag directly:
/// `app.state::<GamepadState>().set_logging(true);`
pub struct GamepadState {
    logging: Arc<AtomicBool>,
    running: Arc<AtomicBool>,
}

impl GamepadState {
    pub fn is_logging(&self) -> bool {
        self.logging.load(Ordering::Relaxed)
    }

    pub fn set_logging(&self, enabled: bool) {
        self.logging.store(enabled, Ordering::Relaxed);
    }
}

fn gamepad_to_json(gamepad: Gamepad, event: EventType, time: SystemTime, log: bool) -> Value {
    // TODO: pull from the device itself
    let num_of_axes: u16 = 9;
    let num_of_buttons: u16 = 20;

    let id = gamepad.id();
    let timestamp = time.duration_since(UNIX_EPOCH).unwrap().as_millis();
    let name = gamepad.name();
    let connected = gamepad.is_connected();

    // TODO: not supported in gilrs yet, but works in sdl2
    let vibration = gamepad.is_ff_supported();

    let uuid = uuid::Uuid::from_bytes(gamepad.uuid())
        .as_hyphenated()
        .to_string();
    let mapping = match gamepad.mapping_source() {
        MappingSource::SdlMappings => "standard",
        _ => "",
    };
    let power_info = gamepad.power_info();

    let axes: Vec<f32> = (0_u16..num_of_axes)
        .map(|idx| gamepad.axis_data(axis_from_u16(idx)))
        .map(|o| match o {
            Some(&axis) => axis.value(),
            None => 0.0,
        })
        .collect();

    let buttons: Vec<f32> = (0_u16..num_of_buttons)
        .map(|idx| gamepad.button_data(button_from_u16(idx)))
        .map(|o| match o {
            Some(button) => button.value(),
            None => 0.0,
        })
        .collect();

    let json = json!({
        "id":id,
        "uuid": uuid,
        "connected": connected,
        "vibration": vibration,
        "event": event,
        "timestamp": timestamp,
        "name": name,
        "buttons": buttons,
        "axes": axes,
        "mapping": mapping,
        "power_info": format!("{:?}",power_info),
    });

    if log {
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
    }

    json
}

#[command]
async fn execute<R: Runtime>(app: AppHandle<R>, _window: Window<R>) {
    let state = app.state::<GamepadState>();
    if state.running.swap(true, Ordering::SeqCst) {
        return;
    }
    let logging = state.logging.clone();
    let mut gilrs = Gilrs::new().unwrap();

    loop {
        let mut had_event = false;
        while let Some(Event { id, event, time, .. }) = gilrs.next_event() {
            had_event = true;
            let gamepad = gilrs.gamepad(id);
            let payload = gamepad_to_json(gamepad, event, time, logging.load(Ordering::Relaxed));
            app.emit_to(EventTarget::any(), "event", payload).unwrap();
        }
        if !had_event {
            std::thread::sleep(std::time::Duration::from_millis(4));
        }
    }
}

#[command]
fn set_logging(state: tauri::State<'_, GamepadState>, enabled: bool) {
    state.set_logging(enabled);
}

#[command]
fn get_logging(state: tauri::State<'_, GamepadState>) -> bool {
    state.is_logging()
}

/// Initializes the plugin. Logging is off by default.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let state = GamepadState {
        logging: Arc::new(AtomicBool::new(false)),
        running: Arc::new(AtomicBool::new(false)),
    };

    Builder::new("gamepad")
        .setup(|app, _| {
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![execute, set_logging, get_logging])
        .build()
}
