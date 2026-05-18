# Changelog

## 0.0.6

- Add `set_logging` / `get_logging` commands so consumers can toggle console output at runtime; logging now defaults to off.
- Fix polling loop pinning the CPU at 100% when idle — the thread now sleeps 4ms after draining an empty event queue (~250 polls/sec).
- Guard against duplicate `execute` calls and phantom axes.
- Default permissions adjusted.

## 0.0.5

- Tauri 2.0.x support.
