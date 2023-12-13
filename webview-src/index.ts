//@ts-ignore
import { listen } from "@tauri-apps/api/event";
//@ts-ignore
import { invoke } from "@tauri-apps/api/primitives";

export type PluginEvent = {
  id: number;
  uuid: string;
  connected: boolean;
  vibration: boolean;
  event: string | object;
  timestamp: number;
  name: string;
  buttons: ReadonlyArray<number>;
  axes: ReadonlyArray<number>;
  mapping: string;
  power_info: string;
};

type UnlistenFn = () => void;
type Callback = (response: PluginEvent) => void;
let callback: Callback | null = null;
let unlisten: Promise<UnlistenFn>;

let gamepads: Gamepad[] = [];

export const getGamepads = () => {
  if (gamepads.length == 0) return [null, null, null, null];
  else return gamepads;
};

const start = () => {
  invoke("plugin:gamepad|execute");
  navigator.getGamepads = getGamepads;

  unlisten = listen<PluginEvent>("event", (event: any) => {
    const { payload } = event;
    let gamepad = eventToGamepad(payload);
    let added = false;
    gamepads = gamepads.map((g) => {
      if (g.id === gamepad.id && g.index === gamepad.index) {
        added = true;
        return gamepad;
      }
      return g;
    });

    if (!added) gamepads.push(gamepad);

    if (payload.event === `Connected`) {
      let event: any = new Event("gamepadconnected");
      event.gamepad = gamepad;
      window.dispatchEvent(event as GamepadEvent);
    }
    if (payload.event === `Disconnected`) {
      let event: any = new Event("gamepaddisconnected");
      event.gamepad = gamepad;
      window.dispatchEvent(event as GamepadEvent);
      gamepads = gamepads.filter((g) => g.index !== gamepad.index);
    }
    if (callback !== null) callback(payload);
  });
};
start();

const eventToGamepad = (event: PluginEvent): Gamepad => {
  const { id, axes, connected, name, timestamp, uuid, mapping } = event;
  const buttons = event.buttons.map(
    (value) => ({ value, touched: false, pressed: value > 0 } as GamepadButton)
  );

  return {
    index: id,
    id: `${name} (${uuid})`,
    connected,
    axes,
    buttons,
    timestamp,
    mapping,
    hapticActuators: [],
    vibrationActuator: null,
  } as Gamepad;
};

export const execute = async (cb: Callback) => {
  callback = cb;

  return unlisten;
};
