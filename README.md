# Tauri Plugin Gamepad

A plugin for [Tauri](https://github.com/tauri-apps/tauri) that provides a polyfill for [Gamepad Web API](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad_API/Using_the_Gamepad_API) that works on most common platforms.

![demo](https://github.com/eugenehp/tauri-plugin-gamepad/raw/master/docs/demo.png)

It's built on top of [gilrs](https://crates.io/crates/gilrs) library.

## Tauri versions support

| Tauri version |  git  |
|---------------|-------|
| 1.x           | [0.0.2](https://github.com/DeveloperMindset-com/tauri-plugin-gamepad/tree/v1)    |
| 2.0.0-beta    | [0.0.3](https://github.com/DeveloperMindset-com/tauri-plugin-gamepad/releases/tag/0.0.3) |
| 2.0.0-rc      | [0.0.4](https://github.com/DeveloperMindset-com/tauri-plugin-gamepad/releases/tag/0.0.4) |
| 2.0.x         | [0.0.5](https://github.com/DeveloperMindset-com/tauri-plugin-gamepad/releases/tag/0.0.5) |

## Why

By default WebKit does not support all the joysticks and gamepads, especially on Safari (macOS and iOS). It does support a bit more in Chrome's version of WebKit, but that's not the one used by Tauri.

Therefore this plugin was created to bridge this gap and provide same access via homogenous Gamepad API that's already available in most browsers.

## Tutorial

[YouTube](https://www.youtube.com/embed/EuGDSnfuGjU?si=0LLJXYvER_acBOWg)
[![youtube tutorial](https://img.youtube.com/vi/EuGDSnfuGjU/maxresdefault.jpg)](https://www.youtube.com/embed/EuGDSnfuGjU?si=0LLJXYvER_acBOWg)


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

## Permissions

Make sure to include proper permissions to enable the plugin to share events with the webpage.

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    "core:app:default",
    "core:event:allow-listen",
    "gamepad:default"
  ]
}
```

## Contributing
When contributing to this repository, please first discuss the change you wish to make via issue, email, or any other method with the owners of this repository before making a change.

## License
[MIT](./LICENSE)

Copyright (c) 2023-2024 Eugene Hauptmann
