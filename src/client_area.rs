use {
    super::{WindowSubclass, Margins},
    std::cell::Cell,
};
#[cfg(windows)]
use winapi::{
    shared::{
        ntdef::NULL,
        minwindef::*,
        windef::*,
    },
    um::{
        winuser::*,
        commctrl::DefSubclassProc,
    },
};

#[derive(Default)]
pub struct ClientArea {
    h_wnd: Cell<Option<HWND>>,
    margins: Cell<Margins>,
}
impl ClientArea {
    pub fn margins(margins: Margins) -> Self {
        Self {
            h_wnd: Cell::new(None),
            margins: Cell::new(margins),
        }
    }
    pub fn set_margins(&self, margins: Margins) {
        self.margins.set(margins);
        unsafe {
            frame_change(self.h_wnd.get().unwrap());
        }
    }
}
impl WindowSubclass for ClientArea {
    #[cfg(windows)]
    fn wnd_proc(
        &self,
        h_wnd: HWND,
        message: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        unsafe {
            // Handle window creation.
            if message == WM_CREATE {
                frame_change(h_wnd);
            }

            // Handle the non-client size message.
            if message == WM_NCCALCSIZE {
                let margins = self.margins.get();

                match w_param as BOOL {
                    TRUE => {
                        // Calculate new NCCALCSIZE_PARAMS based on custom NCA inset.
                        // NCCALCSIZE_PARAMS *pncsp = reinterpret_cast<NCCALCSIZE_PARAMS*>(l_param);
                        let pncsp = &mut *(l_param as *mut NCCALCSIZE_PARAMS);

                        pncsp.rgrc[0].left   -= margins.left;
                        pncsp.rgrc[0].top    -= margins.top;
                        pncsp.rgrc[0].right  += margins.right;
                        pncsp.rgrc[0].bottom += margins.bottom;

                        // return WVR_VALIDRECTS;
                    }
                    FALSE => {
                        let rc = l_param as *mut RECT;
                        let rc = &mut *rc;
                        let rc_window = window_rect(h_wnd);
                        CopyRect(rc, &rc_window);
                        rc.left += margins.left;
                        rc.top += margins.top;
                        rc.right -= margins.right;
                        rc.bottom -= margins.bottom;

                        return 0;
                    }
                    _ => {}
                }
            }
            DefSubclassProc(h_wnd, message, w_param, l_param)
        }
    }
    #[cfg(windows)]
    fn init(&self, h_wnd: HWND) {
        self.h_wnd.set(Some(h_wnd));
        unsafe {
            frame_change(h_wnd);
        }
    }
}

#[cfg(windows)]
unsafe fn frame_change(h_wnd: HWND) {
    let rc_client = window_rect(h_wnd);

    // Inform application of the frame change.
    SetWindowPos(h_wnd,
                 NULL as _,
                 rc_client.left, rc_client.top,
                 rc_client.right - rc_client.left,
                 rc_client.bottom - rc_client.top,
                 SWP_FRAMECHANGED);
}

#[cfg(windows)]
pub(crate) unsafe fn window_rect(h_wnd: HWND) -> RECT {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    GetWindowRect(h_wnd, &mut rect);
    rect
}