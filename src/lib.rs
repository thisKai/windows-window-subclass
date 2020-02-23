#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::subclass_win32_window;
use raw_window_handle::{
    RawWindowHandle, 
    HasRawWindowHandle,
    windows::WindowsHandle,
};


pub fn subclass_window<W: HasRawWindowHandle>(window: &W) {
    #[cfg(windows)]
    {
        if let RawWindowHandle::Windows(WindowsHandle { hwnd, .. }) = window.raw_window_handle() {
            subclass_win32_window(hwnd as _);
        }
    }
}