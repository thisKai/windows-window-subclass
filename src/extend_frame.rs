use {
    super::WindowSubclass,
    std::cell::Cell,
};
#[cfg(windows)]
use winapi::{
    shared::{
        winerror::*,
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

#[derive(Default)]
pub struct ExtendFrame {
    h_wnd: Cell<Option<HWND>>,
    margins: Cell<Margins>,
}
impl ExtendFrame {
    pub fn margins(margins: Margins) -> Self {
        Self {
            h_wnd: Cell::new(None),
            margins: Cell::new(margins),
        }
    }
    pub fn sheet() -> Self {
        Self::margins(Margins::sheet())
    }
    pub fn set_margins(&self, margins: Margins) {
        self.margins.set(margins);
        unsafe {
            extend_frame(
                self.h_wnd.get().unwrap(),
                &self.margins.get().winapi(),
            );
        }
    }
}
impl WindowSubclass for ExtendFrame {
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
    fn init(&self, h_wnd: HWND) {
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

#[derive(Default, Copy, Clone)]
pub struct Margins {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl Margins {
    pub fn sheet() -> Self {
        Self {
            left: -1,
            top: -1,
            right: -1,
            bottom: -1,
        }
    }
    pub(crate) fn winapi(&self) -> MARGINS {
        MARGINS {
            cxLeftWidth: self.left,
            cxRightWidth: self.right,
            cyBottomHeight: self.bottom,
            cyTopHeight: self.top,
        }
    }
}