# Tauri Plugin Gamepad

A plugin for [Tauri](https://github.com/tauri-apps/tauri) that provides a polyfill for [Gamepad Web API](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad_API/Using_the_Gamepad_API) that works on most common platforms.

![demo](https://github.com/eugenehp/tauri-plugin-gamepad/raw/master/docs/demo.png)

It's built on top of [gilrs](https://crates.io/crates/gilrs) library.

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