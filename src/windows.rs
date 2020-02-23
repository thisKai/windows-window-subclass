use {
    winapi::{
        shared::{
            winerror::*,
            basetsd::*,
            ntdef::NULL,
            minwindef::*,
            windef::*,
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

pub fn subclass_window(h_wnd: HWND) {
    unsafe {
        SetWindowSubclass(h_wnd, Some(subclass_wnd_proc), 0, 0);

        extend_frame(h_wnd);
        frame_change(h_wnd);
    }
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
    _dw_ref_data: DWORD_PTR,
) -> LRESULT {
    let mut f_call_dwp = true;
    let mut f_dwm_enabled = FALSE;
    let mut l_ret = 0;

    // Winproc worker for custom frame issues.
    let hr = DwmIsCompositionEnabled(&mut f_dwm_enabled);
    if SUCCEEDED(hr) {
        l_ret = custom_caption_proc(h_wnd, message, w_param, l_param, &mut f_call_dwp);
    }

    // Winproc worker for the rest of the application.
    if f_call_dwp {
        l_ret = DefSubclassProc(h_wnd, message, w_param, l_param);
    }
    l_ret
}

const LEFTEXTENDWIDTH: i32 = 0;
const RIGHTEXTENDWIDTH: i32 = 0;
const BOTTOMEXTENDWIDTH: i32 = 0;
const TOPEXTENDWIDTH: i32 = 31;

unsafe fn window_rect(h_wnd: HWND) {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    GetWindowRect(h_wnd, &mut rect);
    rect
}

unsafe fn frame_change(h_wnd: HWND) {
    let mut rc_client = window_rect();

    // Inform application of the frame change.
    SetWindowPos(h_wnd,
                 NULL as _,
                 rc_client.left, rc_client.top,
                 rc_client.right - rc_client.left,
                 rc_client.bottom - rc_client.top,
                 SWP_FRAMECHANGED);
}

unsafe fn extend_frame(h_wnd: HWND) {
    // Extend the frame into the client area.
    let margins = MARGINS {
        cxLeftWidth: LEFTEXTENDWIDTH,
        cxRightWidth: RIGHTEXTENDWIDTH,
        cyBottomHeight: BOTTOMEXTENDWIDTH,
        cyTopHeight: TOPEXTENDWIDTH,
    };

    let hr = DwmExtendFrameIntoClientArea(h_wnd, &margins);

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
) -> LRESULT{
    let mut l_ret = 0;
    let mut f_call_dwp = true; // Pass on to DefWindowProc?

    f_call_dwp = DwmDefWindowProc(h_wnd, message, w_param, l_param, &mut l_ret) != TRUE;

    // Handle window creation.
    if message == WM_CREATE {
        frame_change(h_wnd);

        f_call_dwp = true;
        l_ret = 0;
    }

    // Handle window activation.
    if message == WM_ACTIVATE {
        // Extend the frame into the client area.
        extend_frame(h_wnd);

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
    if message == WM_NCCALCSIZE && w_param as BOOL == TRUE {
        // Calculate new NCCALCSIZE_PARAMS based on custom NCA inset.
        // NCCALCSIZE_PARAMS *pncsp = reinterpret_cast<NCCALCSIZE_PARAMS*>(l_param);
        let pncsp = &mut *(l_param as *mut NCCALCSIZE_PARAMS);

        pncsp.rgrc[0].left   -= 0;
        pncsp.rgrc[0].top    -= 31;
        pncsp.rgrc[0].right  += 0;
        pncsp.rgrc[0].bottom += 0;

        l_ret = 0;

        // No need to pass the message on to the DefWindowProc.
        // f_call_dwp = false;
    }

    // Handle hit testing in the NCA if not handled by DwmDefWindowProc.
    if message == WM_NCHITTEST && l_ret == 0 {
        l_ret = match DefSubclassProc(h_wnd, message, w_param, l_param) {
            HTCLIENT => HTCAPTION,
            ret => ret,
        };

        if l_ret != HTNOWHERE {
            f_call_dwp = false;
        }
    }

    *pf_call_dwp = f_call_dwp;

    l_ret
}
