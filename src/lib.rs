#[cfg(windows)]
mod subclass;
mod metrics;
mod dwm_frame;
#[cfg(feature = "unstable")]
mod client_area;
#[cfg(feature = "unstable")]
mod hit_test;

#[cfg(windows)]
pub use {
    metrics::{
        window_frame_metrics,
        WindowFrameMetrics,
        Margins,
    },
    dwm_frame::DwmFrame,
    subclass::subclass_win32_window,
};
#[cfg(feature = "unstable")]
pub use {
    client_area::ClientArea,
    hit_test::HitTest,
};

use std::ops::Deref;
#[cfg(windows)]
use {
    winapi::shared::{
        basetsd::*,
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
    fn init(&self, _h_wnd: HWND, _u_id_subclass: UINT_PTR) {}
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
    fn init(&self, h_wnd: HWND, u_id_subclass: UINT_PTR) {
        self.deref().init(h_wnd, u_id_subclass)
    }
}

pub trait SetSubclass {
    fn set_subclass<S: WindowSubclass>(&self, subclass: S);
    fn with_subclass<S: WindowSubclass>(self, subclass: S) -> Self where Self: Sized {
        self.set_subclass(subclass);
        self
    }
}
impl<W: HasRawWindowHandle> SetSubclass for W {
    fn set_subclass<S: WindowSubclass>(&self, subclass: S) {
        #[cfg(windows)]
        {
            use raw_window_handle::{
                RawWindowHandle,
                windows::WindowsHandle,
            };
            if let RawWindowHandle::Windows(WindowsHandle { hwnd, .. }) = self.raw_window_handle() {
                subclass_win32_window(hwnd as _, subclass);
            }
        }
    }
}
