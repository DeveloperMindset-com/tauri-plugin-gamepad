'use strict';

var event = require('@tauri-apps/api/event');
var core = require('@tauri-apps/api/core');

let callback = null;
let unlisten;
let gamepads = [];
const getGamepads = () => {
    if (gamepads.length == 0)
        return [null, null, null, null];
    else
        return gamepads;
};
const start = () => {
    core.invoke("plugin:gamepad|execute");
    navigator.getGamepads = getGamepads;
    unlisten = event.listen("event", (event) => {
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
        if (!added)
            gamepads.push(gamepad);
        if (payload.event === `Connected`) {
            let event = new Event("gamepadconnected");
            event.gamepad = gamepad;
            window.dispatchEvent(event);
        }
        if (payload.event === `Disconnected`) {
            let event = new Event("gamepaddisconnected");
            event.gamepad = gamepad;
            window.dispatchEvent(event);
            gamepads = gamepads.filter((g) => g.index !== gamepad.index);
        }
        if (callback !== null)
            callback(payload);
    });
};
start();
const eventToGamepad = (event) => {
    const { id, axes, connected, name, timestamp, uuid, mapping } = event;
    const buttons = event.buttons.map((value) => ({ value, touched: false, pressed: value > 0 }));
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
    };
};
const execute = async (cb) => {
    callback = cb;
    return unlisten;
};

exports.execute = execute;
exports.getGamepads = getGamepads;
