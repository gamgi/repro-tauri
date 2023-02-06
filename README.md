# Repro: Tauri iframes communication issue

This is a reproduction of iframe communication issue with Tauri.

## How to reproduce

0. Ensure you have installed tauri and prerequisites.
1. Clone repo at https://github.com/gamgi/repro-tauri and enter its root.
2. Install prerequisites `npm install`
3. Start the tauri app `cargo tauri dev`
4. In parent greet box, type name and press greet (it works)
5. In child greet box, type name and press greet (it does not work)
6. Open console, see error `[TAURI] Couldn't find callback id 90962087 in window. This happens when the app is reloaded while Rust is running an asynchronous operation.`
