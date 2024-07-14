use tauri::{Emitter, EventTarget};

use gilrs::{Event, EventType, Gamepad, Gilrs, MappingSource};

use serde_json::{json, Value};

use std::time::{SystemTime, UNIX_EPOCH};
use std::u16;

use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    AppHandle, Runtime, Window,
};

mod utils;
use crate::utils::{axis_from_u16, button_from_u16};

// TODO: pull from the device itself
const NUM_OF_AXES: u16 = 12;
const NUM_OF_BUTTONS: u16 = 20;

fn gamepad_to_json(gamepad: Gamepad, event: EventType, time: SystemTime) -> Value {
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

    let axes: Vec<f32> = (0 as u16..NUM_OF_AXES)
        .map(|idx| gamepad.axis_data(axis_from_u16(idx)))
        .map(|o| match o {
            Some(&axis) => axis.value(),
            None => 0.0,
        })
        .collect();

    let buttons: Vec<f32> = (0 as u16..NUM_OF_BUTTONS)
        .map(|idx| gamepad.button_data(button_from_u16(idx)))
        .map(|o| match o {
            Some(button) => button.value(),
            None => 0.0,
        })
        .collect();

    json!({
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
    })
}

#[command]
async fn execute<R: Runtime>(app: AppHandle<R>, _window: Window<R>) {
    println!("execute");
    let mut gilrs = Gilrs::new().unwrap();

    loop {
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            let gamepad = gilrs.gamepad(id);
            let payload = gamepad_to_json(gamepad, event, time);
            app.emit_to(EventTarget::any(), "event", payload).unwrap();
        }
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("gamepad")
        .invoke_handler(tauri::generate_handler![execute])
        .build()
}
