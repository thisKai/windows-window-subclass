use {
    super::{WindowSubclass, client_area::window_rect},
    std::cell::Cell,
};
#[cfg(windows)]
use winapi::{
    shared::{
        ntdef::NULL,
        minwindef::*,
        windef::*,
        windowsx::*,
    },
    um::{
        winuser::*,
        commctrl::DefSubclassProc,
    },
};

#[derive(Default)]
pub struct HitTest {
    h_wnd: Cell<Option<HWND>>,
    titlebar_height: Cell<i32>,
}
impl HitTest {
    pub fn extend_titlebar(titlebar_height: i32) -> Self {
        Self {
            h_wnd: Cell::new(None),
            titlebar_height: Cell::new(titlebar_height),
        }
    }
    pub fn set_titlebar_height(&self, titlebar_height: i32) {
        self.titlebar_height.set(titlebar_height);
    }
    unsafe fn hit_test(&self, h_wnd: HWND, l_param: LPARAM) -> LRESULT {
        let window = window_rect(h_wnd);
        let frame = window_frame_rect();
        dbg!(&frame.top);
        let POINT { y, .. } = pointer_location(l_param);

        if y >= window.top && y < window.top + self.titlebar_height.get() {
            if y < (window.top - frame.top) {
                HTTOP
            } else {
                HTCAPTION
            }
        } else {
            HTCLIENT
        }
    }
}
impl WindowSubclass for HitTest {
    #[cfg(windows)]
    fn wnd_proc(
        &self,
        h_wnd: HWND,
        message: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        unsafe {
            let default_ret = DefSubclassProc(h_wnd, message, w_param, l_param);

            if message == WM_NCHITTEST {
                let hit_test = match default_ret {
                    HTCLIENT => self.hit_test(h_wnd, l_param),
                    value => value,
                };

                if hit_test != HTNOWHERE {
                    return hit_test;
                }
            }
            default_ret
        }
    }
    #[cfg(windows)]
    fn init(&self, h_wnd: HWND) {
        self.h_wnd.set(Some(h_wnd));
    }
}

#[cfg(windows)]
unsafe fn window_frame_rect() -> RECT {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    AdjustWindowRectEx(&mut rect, WS_OVERLAPPEDWINDOW & !WS_CAPTION, FALSE, NULL as _);
    rect
}

#[cfg(windows)]
unsafe fn pointer_location(l_param: LPARAM) -> POINT {
    POINT {
        x: GET_X_LPARAM(l_param),
        y: GET_Y_LPARAM(l_param),
    }
}
