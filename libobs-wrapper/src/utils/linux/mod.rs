//! Contains linux specific bindings to x11 and wayland

use std::{fs, os::raw::c_char};
use std::path::PathBuf;

extern "C" {
    // X11 functions
    pub(crate) fn XOpenDisplay(display_name: *const c_char) -> *mut std::os::raw::c_void;
    pub(crate) fn XCloseDisplay(display: *mut std::os::raw::c_void) -> i32;

    // Wayland functions
    pub(crate) fn wl_display_connect(name: *const c_char) -> *mut std::os::raw::c_void;
    pub(crate) fn wl_display_disconnect(display: *mut std::os::raw::c_void);
}
#[derive(Debug)]
pub struct LinuxGlibLoop {
    glib_loop: glib::MainLoop,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl LinuxGlibLoop {
    pub fn new() -> Self {
        let g_loop = glib::MainLoop::new(None, false);
        let g_loop_clone = g_loop.clone();
        let handle = std::thread::spawn(move || {
            g_loop_clone.run();
        });

        Self {
            glib_loop: g_loop,
            handle: Some(handle),
        }
    }
}

impl Default for LinuxGlibLoop {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for LinuxGlibLoop {
    fn drop(&mut self) {
        if self.glib_loop.is_running() {
            self.glib_loop.quit();
        }

        if let Some(handle) = self.handle.take() {
            let r = handle.join();
            if std::thread::panicking() {
                if let Err(e) = r {
                    log::error!(
                        "[libobs-wrapper]: Thread panicked while dropping LinuxGlibLoop: {:?}",
                        e
                    );
                }
            } else {
                r.unwrap();
            }
        }
    }
}

pub(crate) fn wl_proxy_get_display(
    proxy: *mut std::os::raw::c_void,
) -> Result<*mut std::os::raw::c_void, libloading::Error> {
    unsafe {
        let lib = libloading::Library::new("libwayland-client.so")
            .or_else(|_e| libloading::Library::new("libwayland-client.so.0"))?;
        let sym: Result<
            libloading::Symbol<
                unsafe extern "C" fn(*mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
            >,
            libloading::Error,
        > = lib.get(b"wl_proxy_get_display\0");

        match sym {
            Ok(f) => Ok(f(proxy)),
            Err(e) => Err(e),
        }
    }
}

/// We are trying to get the correct OpenGL library name for Linux systems derived from the obs binary, this is a bit hacky.
pub(crate) fn find_obs_binary() -> PathBuf {
    let path_env = std::env::var("PATH").ok();
    let mut preferred: Option<PathBuf> = None;
    let mut fallback: Option<PathBuf> = None;

    if let Some(path) = path_env {
        for dir in path.split(':') {
            let base = PathBuf::from(dir);

            // Prefer ".obs-wrapped" if any ancestor folder is "nix"
            let wrapped = base.join(".obs-wrapped");
            if wrapped.exists() {
                let has_nix_parent = wrapped
                    .ancestors()
                    .any(|a| a.file_name().map(|n| n == "nix").unwrap_or(false));
                if has_nix_parent {
                    preferred = Some(wrapped);
                    break;
                }
            }

            // Fallback to "obs"
            let obs = base.join("obs");
            if fallback.is_none() && obs.exists() {
                fallback = Some(obs);
            }
        }
    }

    preferred.or(fallback).unwrap_or_else(|| PathBuf::from("/usr/bin/obs"))
}

pub(crate) fn get_linux_opengl_lib_name() -> String {
    let obs_bin = find_obs_binary();
    let obs_bin = obs_bin.to_str().unwrap_or("/usr/bin/obs");

    if !std::path::Path::new(obs_bin).exists() {
        log::debug!(
            "Couldn't find obs binary at {}, using fallback OpenGL lib name.",
            obs_bin
        );
        return "libobs-opengl.so".to_string(); // Fallback
    }

    let raw_strings = fs::read(obs_bin).unwrap_or_default();
    let search_str = "libobs-opengl";
    let search_bytes = search_str.as_bytes();
    let found_lib_name = "libobs-opengl.so".to_string(); // Fallback

    let mut idx = 0usize;
    while idx < raw_strings.len() {
        // Find next occurrence of "libobs-opengl"
        let rel = raw_strings[idx..]
            .windows(search_bytes.len())
            .position(|w| w == search_bytes);
        let start = match rel {
            Some(r) => idx + r,
            None => break,
        };

        // Extract bytes until the next NUL (C-string terminator) or EOF
        let end = match raw_strings[start..].iter().position(|&b| b == 0) {
            Some(p) => start + p,
            None => raw_strings.len(),
        };

        if end > start {
            if let Ok(s) = std::str::from_utf8(&raw_strings[start..end]) {
                if s.contains(".so") {
                    return s.to_string();
                }
            }
        }

        // Continue search after this occurrence
        idx = start + search_bytes.len();
    }

    log::debug!("Extracted OpenGL lib name: {}", found_lib_name);
    found_lib_name
}
