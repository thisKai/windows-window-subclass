#[cfg(windows)]
mod subclass;
#[cfg(feature = "unstable")]
mod custom_window;
#[cfg(feature = "unstable")]
mod extend_frame;
#[cfg(feature = "unstable")]
mod client_area;

#[cfg(windows)]
pub use subclass::subclass_win32_window;
#[cfg(feature = "unstable")]
pub use {
    custom_window::CustomWindow,
    extend_frame::{ExtendFrame, Margins},
    client_area::ClientArea,
};

use std::ops::Deref;
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
    fn wnd_proc(
        &self,
        h_wnd: HWND,
        message: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT;
    #[cfg(windows)]
    fn init(&self, h_wnd: HWND) {}
}
impl<T: Deref> WindowSubclass for T where T::Target: WindowSubclass {
    #[cfg(windows)]
    fn wnd_proc(
        &self,
        h_wnd: HWND,
        message: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        self.deref().wnd_proc(h_wnd, message, w_param, l_param)
    }
    #[cfg(windows)]
    fn init(&self, h_wnd: HWND) {
        self.deref().init(h_wnd)
    }
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
