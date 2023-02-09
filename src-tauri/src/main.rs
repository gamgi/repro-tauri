#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("Rust backend called with {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Vanilla tauri responder
/// from core/tauri/src/hooks.rs
pub fn window_invoke_responder<R: tauri::Runtime>(
    window: tauri::Window<R>,
    response: tauri::InvokeResponse,
    success_callback: tauri::api::ipc::CallbackFn,
    error_callback: tauri::api::ipc::CallbackFn,
) {
    let callback_string = match tauri::api::ipc::format_callback_result(
        response.into_result(),
        success_callback,
        error_callback,
    ) {
        Ok(callback_string) => callback_string,
        Err(e) => tauri::api::ipc::format_callback(error_callback, &e.to_string())
            .expect("unable to serialize response string to json"),
    };

    let _ = window.eval(&callback_string);
}

/// Custom iframe-aware initialization script
#[rustfmt::skip]
fn invoke_initialization_script() -> String {
    const STRINGIFY_IPC_MESSAGE_FN: &str = "JSON.stringify";

    // Custom iframe transformCallback proxy based on `core/tauri/scripts/core.js`
    let add_iframe_proxy = format!(r#"
        // Check if invoked within iframe
        if (window.frameElement) {{
          const {{callback, error}} = message;
          
          // Add proxies from parent window to child window
          // as-if transformCallback() was called on parent
          [callback, error].map(id => `_${{id}}`).map(prop =>
            Object.defineProperty(window.parent, prop, {{
              value: (result) => {{

                // Remove proxies from parent
                Reflect.deleteProperty(window.parent, `_${{error}}`)
                Reflect.deleteProperty(window.parent, `_${{callback}}`)

                // Proxy result to child
                return window[prop](result)
              }},
              writable: false,
              configurable: true
            }})
          )
        }}
    "#);

    // Vanilla invoke initialization script from `core/tauri/src/app.rs`
    format!(r#"
    Object.defineProperty(window, '__TAURI_POST_MESSAGE__', {{
      value: (message) => {{
        {add_iframe_proxy}
        window.ipc.postMessage({STRINGIFY_IPC_MESSAGE_FN}(message))
      }}
    }})
    "#)
    .to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_system(invoke_initialization_script(), window_invoke_responder)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
