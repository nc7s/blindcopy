#![doc = include_str!("../README.md")]

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub use linux::text;
#[cfg(target_os = "macos")]
pub use macos::text;
#[cfg(target_os = "windows")]
pub use windows::text;
