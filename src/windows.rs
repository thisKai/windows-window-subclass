use {
    winapi::{
        shared::{
            winerror::*,
            basetsd::*,
            ntdef::NULL,
            minwindef::*,
            windef::*,
            windowsx::*,
        },
        um::{
            winuser::*,
            commctrl::{DefSubclassProc, SetWindowSubclass},
            uxtheme::MARGINS,
            dwmapi::{
                DwmDefWindowProc,
                DwmIsCompositionEnabled,
                DwmExtendFrameIntoClientArea,
            },
        },
    },
};

pub trait WindowSubclass {
    fn subclass_wnd_proc(
        &mut self,
        h_wnd: HWND,
        message: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
        u_id_subclass: UINT_PTR,
    ) -> LRESULT;
}
pub fn apply_subclass<S: WindowSubclass>(h_wnd: HWND, subclass: S) {
    let data = Box::new(subclass);
    unsafe {
        SetWindowSubclass(h_wnd, Some(subclass_wnd_proc), 0, data as _);
    }
}

pub struct Options {
    margins: MARGINS,
}
impl Default for Options {
    fn default() -> Self {
        Options {
            margins: MARGINS {
                cxLeftWidth: 0,
                cxRightWidth: 0,
                cyBottomHeight: 0,
                cyTopHeight: 31,
            },
        }
    }
}

pub fn subclass_win32_window(h_wnd: HWND, options: Options) {
    let data = create_data(options);
    unsafe {
        SetWindowSubclass(h_wnd, Some(subclass_wnd_proc), 0, data as _);

        let data = &*data;
        extend_frame(h_wnd, &data.margins);
        frame_change(h_wnd);
    }
}

fn create_data(data: Options) -> *mut Options {
    let data = Box::new(data);
    Box::into_raw(data)
}
unsafe fn get_data<'a>(dw_ref_data: DWORD_PTR) -> &'a Options {
    let data = dw_ref_data as *mut Options;
    &*data
}

//
//  Subclass WinProc.
//
unsafe extern "system" fn subclass_wnd_proc(
    h_wnd: HWND,
    message: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
    _u_id_subclass: UINT_PTR,
    dw_ref_data: DWORD_PTR,
) -> LRESULT {
    let data = get_data(dw_ref_data);
    let mut f_call_dwp = true;
    let mut f_dwm_enabled = FALSE;
    let mut l_ret = 0;

    // Winproc worker for custom frame issues.
    let hr = DwmIsCompositionEnabled(&mut f_dwm_enabled);
    if SUCCEEDED(hr) {
        l_ret = custom_caption_proc(
            h_wnd,
            message,
            w_param,
            l_param,
            &mut f_call_dwp,
            data,
        );
    }

    // Winproc worker for the rest of the application.
    if f_call_dwp {
        l_ret = DefSubclassProc(h_wnd, message, w_param, l_param);
    }
    l_ret
}

unsafe fn window_rect(h_wnd: HWND) -> RECT {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    GetWindowRect(h_wnd, &mut rect);
    rect
}

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

unsafe fn pointer_location(l_param: LPARAM) -> POINT {
    POINT {
        x: GET_X_LPARAM(l_param),
        y: GET_Y_LPARAM(l_param),
    }
}

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

unsafe fn extend_frame(h_wnd: HWND, margins: &MARGINS) {
    // Extend the frame into the client area.
    let hr = DwmExtendFrameIntoClientArea(h_wnd, margins);

    if !SUCCEEDED(hr) {
        // Handle error.
    }
}
//
// Message handler for handling the custom caption messages.
//
unsafe fn custom_caption_proc(
    h_wnd: HWND,
    message: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
    pf_call_dwp: &mut bool,
    data: &Options,
) -> LRESULT{
    let mut l_ret = 0;

    // Pass on to DefWindowProc?
    let mut f_call_dwp = DwmDefWindowProc(h_wnd, message, w_param, l_param, &mut l_ret) != TRUE;

    // Handle window creation.
    if message == WM_CREATE {
        frame_change(h_wnd);

        f_call_dwp = true;
        l_ret = 0;
    }

    // Handle window activation.
    if message == WM_ACTIVATE {
        // Extend the frame into the client area.
        extend_frame(h_wnd, &data.margins);

        f_call_dwp = true;
        l_ret = 0;
    }

    // if (message == WM_PAINT) {
    //     HDC hdc;
    //     {
    //         PAINTSTRUCT ps;
    //         hdc = BeginPaint(h_wnd, &ps);
    //         PaintCustomCaption(h_wnd, hdc);
    //         EndPaint(h_wnd, &ps);
    //     }

    //     f_call_dwp = true;
    //     l_ret = 0;
    // }

    // Handle the non-client size message.
    if message == WM_NCCALCSIZE {
        match w_param as BOOL {
            TRUE => {
                // Calculate new NCCALCSIZE_PARAMS based on custom NCA inset.
                // NCCALCSIZE_PARAMS *pncsp = reinterpret_cast<NCCALCSIZE_PARAMS*>(l_param);
                let pncsp = &mut *(l_param as *mut NCCALCSIZE_PARAMS);

                pncsp.rgrc[0].left   -= 0;
                pncsp.rgrc[0].top    -= 31;
                pncsp.rgrc[0].right  += 0;
                pncsp.rgrc[0].bottom += 0;

                l_ret = WVR_VALIDRECTS;

                // No need to pass the message on to the DefWindowProc.
                // f_call_dwp = false;
            }
            FALSE => {
                let rc = l_param as *mut RECT;
                let rc = &mut *rc;
                let rc_window = window_rect(h_wnd);
                CopyRect(rc, &rc_window);
                rc.left += 0;
                rc.top += 31;
                rc.right -= 0;
                rc.bottom -= 0;

                l_ret = 0;
            }
            _ => {}
        }
    }

    // Handle hit testing in the NCA if not handled by DwmDefWindowProc.
    if message == WM_NCHITTEST && l_ret == 0 {
        l_ret = match DefSubclassProc(h_wnd, message, w_param, l_param) {
            HTCLIENT => hit_test(h_wnd, l_param, data),
            ret => ret,
        };

        if l_ret != HTNOWHERE {
            f_call_dwp = false;
        }
    }

    *pf_call_dwp = f_call_dwp;

    l_ret
}

unsafe fn hit_test(h_wnd: HWND, l_param: LPARAM, data: &Options) -> LRESULT {
    let window = window_rect(h_wnd);
    let frame = window_frame_rect();
    dbg!(&frame.top);
    let POINT { y, .. } = pointer_location(l_param);

    if y >= window.top && y < window.top + data.margins.cyTopHeight {
        if y < (window.top - frame.top) {
            HTTOP
        } else {
            HTCAPTION
        }
    } else {
        HTCLIENT
    }
}
