# Repro: Tauri iframes communication issue

This is a reproduction of iframe communication issue with Tauri.

## How to reproduce

1. Ensure you have installed tauri and prerequisites.
2. run `cargo tauri dev` (or npm equivalent)
2. in parent greet box, type name and press greet (it works)
2. in child greet box, type name and press greet (it does not work)
