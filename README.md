# Tauri Plugin Gamepad

A plugin for [Tauri](https://github.com/tauri-apps/tauri) that provides a polyfill for [Gamepad Web API](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad_API/Using_the_Gamepad_API) that works on most common platforms.

![demo](https://github.com/eugenehp/tauri-plugin-gamepad/raw/master/docs/demo.png)

It's built on top of [gilrs](https://crates.io/crates/gilrs) library.

This plugin supports Tauri `1.x`, for `2.x` support check [v2 branch](https://github.com/eugenehp/tauri-plugin-gamepad/tree/v2).

## Why

By default WebKit does not support all the joysticks and gamepads, especially on Safari (macOS and iOS). It does support a bit more in Chrome's version of WebKit, but that's not the one used by Tauri.

Therefor this plugin was created to bridge this gap and provide same access via homogenous Gamepad API that's already available in most browsers.


## Usage

On the javascript side, simply call `import 'tauri-plugin-gamepad-api'`.

On the rust side, add `.plugin(tauri_plugin_gamepad::init())` to your main's Tauri builder call.

See [example](./examples/tauri-app/) for more.

## Known issues

- Haptic funcionality is not available (because `gilrs` doesn't support it, though `sdl2` does).
- You'll get double `ongamepadconnected` for some of the devices (one from native implementation, and one from this plugin).

## Packages

- `tauri-plugin-gamepad` on [crates](https://crates.io/crates/tauri-plugin-gamepad)
- `tauri-plugin-gamepad-api` on [npm](https://www.npmjs.com/package/tauri-plugin-gamepad-api)

## Supported platforms (from `gilrs`)

|                  | Input | Hotplugging | Force feedback |
|------------------|:-----:|:-----------:|:--------------:|
| Linux/BSD (evdev)|   ✓   |      ✓      |        ✓       |
| Windows (XInput) |   ✓   |      ✓      |        ✓       |
| macOS            |   ✓   |      ✓      |        ✕       |
| iOS              |   ✓   |      ✓      |        ✕       |
| Android          |   ✕   |      ✕      |        ✕       |

## Development

Run `npm run watch` to transpile TypeScript API layer.

Run example demo using these commands:
```shell
cd examples/tauri-app
npm run tauri dev
```

## Contributing
When contributing to this repository, please first discuss the change you wish to make via issue, email, or any other method with the owners of this repository before making a change.

## License
[MIT](./LICENSE)

Copyright (c) 2023 Eugene Hauptmann
