//! Access the system shell. Allows you to spawn child processes and manage files and URLs using their default application.
//!
//! The APIs must be added to `tauri.allowlist.shell` in `tauri.conf.json`:
//!
//! ```json
//! {
//!   "tauri": {
//!     "allowlist": {
//!       "shell": {
//!         "all": true, // enable all shell APIs
//!         "execute": true, // enable process spawn APIs
//!         "sidecar": true, // enable spawning sidecars
//!         "open": true // enable opening files/URLs using the default program
//!       }
//!     }
//!   }
//! }
//! ```
//! It is recommended to allowlist only the APIs you use for optimal bundle size and security.

/// Opens a path or URL with the system's default app.
#[inline(always)]
pub async fn open(path: impl AsRef<str>) -> crate::Result<()> {
    inner::open(path.as_ref(), None).await?;
    Ok(())
}

/// Opens a path or URL with the system's default app, or the one specified with `openWith`.
#[inline(always)]
pub async fn open_with(path: impl AsRef<str>, with: impl Into<Option<&str>>) -> crate::Result<()> {
    inner::open(path.as_ref(), with.into()).await?;
    Ok(())
}

mod inner {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/shell.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn open(path: &str, with: Option<&str>) -> Result<(), JsValue>;
    }
}
