#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::subclass_win32_window;
use raw_window_handle::HasRawWindowHandle;


pub fn subclass_window<W: HasRawWindowHandle>(window: &W) {
    #[cfg(windows)]
    {
        use raw_window_handle::{
            RawWindowHandle,
            windows::WindowsHandle,
        };
        if let RawWindowHandle::Windows(WindowsHandle { hwnd, .. }) = window.raw_window_handle() {
            subclass_win32_window(hwnd as _);
        }
    }
}
