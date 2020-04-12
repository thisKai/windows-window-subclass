use {
    super::{WindowSubclass, Margins},
    std::cell::Cell,
};
#[cfg(windows)]
use winapi::{
    shared::{
        winerror::*,
        basetsd::*,
        minwindef::*,
        windef::*,
    },
    um::{
        winuser::*,
        commctrl::DefSubclassProc,
        uxtheme::MARGINS,
        dwmapi::{
            DwmIsCompositionEnabled,
            DwmExtendFrameIntoClientArea,
        },
    },
};

#[cfg(not(windows))]
type HWND = ();

pub struct DwmFrame {
    h_wnd: Cell<Option<HWND>>,
    margins: Cell<Margins>,
}
impl DwmFrame {
    pub fn extend(margins: Margins) -> Self {
        Self {
            #[cfg(not(windows))]
            h_wnd: Cell::new(None),
            margins: Cell::new(margins),
        }
    }
    pub fn sheet() -> Self {
        Self::extend(Margins::sheet())
    }
    pub fn set_margins(&self, margins: Margins) {
        self.margins.set(margins);
        #[cfg(windows)]
        unsafe {
            extend_frame(
                self.h_wnd.get().unwrap(),
                &self.margins.get().winapi(),
            );
        }
    }
}
impl WindowSubclass for DwmFrame {
    #[cfg(windows)]
    fn wnd_proc(
        &self,
        h_wnd: HWND,
        message: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        unsafe {
            let mut f_dwm_enabled = FALSE;
            let hr = DwmIsCompositionEnabled(&mut f_dwm_enabled);

            // Handle window activation if dwm is enabled
            if message == WM_ACTIVATE && SUCCEEDED(hr) && f_dwm_enabled == TRUE {
                // Extend the frame into the client area.
                extend_frame(h_wnd, &self.margins.get().winapi());
            }
            DefSubclassProc(h_wnd, message, w_param, l_param)
        }
    }
    #[cfg(windows)]
    fn init(&self, h_wnd: HWND, _u_id_subclass: UINT_PTR) {
        self.h_wnd.set(Some(h_wnd));
        unsafe {
            extend_frame(h_wnd, &self.margins.get().winapi());
        }
    }
}

#[cfg(windows)]
unsafe fn extend_frame(h_wnd: HWND, margins: &MARGINS) {
    // Extend the frame into the client area.
    let hr = DwmExtendFrameIntoClientArea(h_wnd, margins);

    if !SUCCEEDED(hr) {
        // Handle error.
    }
}
