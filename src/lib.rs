#[cfg(windows)]
mod subclass;
mod custom_window;

#[cfg(windows)]
pub use {
    subclass::subclass_win32_window,
    custom_window::CustomWindow,
};
#[cfg(windows)]
use {
    winapi::shared::{
        minwindef::*,
        windef::*,
    },
};
use raw_window_handle::HasRawWindowHandle;

pub trait WindowSubclass {
    #[cfg(windows)]
    unsafe fn wnd_proc(
        &mut self,
        h_wnd: HWND,
        message: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT;
    #[cfg(windows)]
    unsafe fn init(&mut self, h_wnd: HWND) {}
}

pub fn subclass_window<W: HasRawWindowHandle, S: WindowSubclass>(window: &W, subclass: S) {
    #[cfg(windows)]
    {
        use raw_window_handle::{
            RawWindowHandle,
            windows::WindowsHandle,
        };
        if let RawWindowHandle::Windows(WindowsHandle { hwnd, .. }) = window.raw_window_handle() {
            subclass_win32_window(hwnd as _, subclass);
        }
    }
}

