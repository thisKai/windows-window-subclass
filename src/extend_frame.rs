use super::WindowSubclass;
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

pub struct ExtendFrame {
    #[cfg(windows)]
    margins: MARGINS,
}
impl Default for ExtendFrame {
    fn default() -> Self {
        ExtendFrame {
            #[cfg(windows)]
            margins: MARGINS {
                cxLeftWidth: -1,
                cxRightWidth: -1,
                cyBottomHeight: -1,
                cyTopHeight: -1,
            },
        }
    }
}
impl WindowSubclass for ExtendFrame {
    #[cfg(windows)]
    fn wnd_proc(
        &mut self,
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
                extend_frame(h_wnd, &self.margins);
            }
            DefSubclassProc(h_wnd, message, w_param, l_param)
        }
    }
    #[cfg(windows)]
    fn init(&mut self, h_wnd: HWND) {
        unsafe {
            extend_frame(h_wnd, &self.margins);
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