use {
    winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        platform::windows::{WindowExtWindows, WindowBuilderExtWindows},
    },
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

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_transparent(true)
        // .with_no_redirection_bitmap(true)
        .build(&event_loop)
        .unwrap();
    unsafe {
        let hWnd = window.hwnd() as _;
        SetWindowSubclass(hWnd, Some(subclass_wnd_proc), 0, 0);

        extend_frame(hWnd);
        frame_change(hWnd);
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

unsafe extern "system" fn subclass_wnd_proc(
    hWnd: HWND,
    uMsg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    uIdSubclass: UINT_PTR,
    dwRefData: DWORD_PTR,
) -> LRESULT {
    wnd_proc(hWnd, uMsg, wParam, lParam)
}

//
//  Main WinProc.
//
pub unsafe extern "system" fn wnd_proc(
    h_wnd: HWND,
    message: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    let mut fCallDWP = true;
    let mut fDwmEnabled = FALSE;
    let mut lRet = 0;
    let mut hr = S_OK;

    // Winproc worker for custom frame issues.
    hr = DwmIsCompositionEnabled(&mut fDwmEnabled);
    if SUCCEEDED(hr) {
        lRet = CustomCaptionProc(h_wnd, message, w_param, l_param, &mut fCallDWP);
    }

    // Winproc worker for the rest of the application.
    if fCallDWP {
        lRet = DefSubclassProc(h_wnd, message, w_param, l_param);
    }
    lRet
}

const LEFTEXTENDWIDTH: i32 = 0;
const RIGHTEXTENDWIDTH: i32 = 0;
const BOTTOMEXTENDWIDTH: i32 = 0;
const TOPEXTENDWIDTH: i32 = 31;


unsafe fn frame_change(hWnd: HWND) {
    let mut rcClient = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    GetWindowRect(hWnd, &mut rcClient);

    // Inform application of the frame change.
    SetWindowPos(hWnd,
                 NULL as _,
                 rcClient.left, rcClient.top,
                 rcClient.right - rcClient.left,
                 rcClient.bottom - rcClient.top,
                 SWP_FRAMECHANGED);
}

unsafe fn extend_frame(hWnd: HWND) {
    // Extend the frame into the client area.
    let margins = MARGINS {
        cxLeftWidth: LEFTEXTENDWIDTH,
        cxRightWidth: RIGHTEXTENDWIDTH,
        cyBottomHeight: BOTTOMEXTENDWIDTH,
        cyTopHeight: TOPEXTENDWIDTH,
    };

    let hr = DwmExtendFrameIntoClientArea(hWnd, &margins);

    if !SUCCEEDED(hr) {
        // Handle error.
    }
}
//
// Message handler for handling the custom caption messages.
//
unsafe fn CustomCaptionProc(
    hWnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    pfCallDWP: &mut bool,
) -> LRESULT{
    let mut lRet = 0;
    let mut hr = S_OK;
    let mut fCallDWP = true; // Pass on to DefWindowProc?

    fCallDWP = DwmDefWindowProc(hWnd, message, wParam, lParam, &mut lRet) != TRUE;

    // Handle window creation.
    if message == WM_CREATE {
        frame_change(hWnd);

        fCallDWP = true;
        lRet = 0;
    }

    // Handle window activation.
    if message == WM_ACTIVATE {
        // Extend the frame into the client area.
        extend_frame(hWnd);

        fCallDWP = true;
        lRet = 0;
    }

    // if (message == WM_PAINT) {
    //     HDC hdc;
    //     {
    //         PAINTSTRUCT ps;
    //         hdc = BeginPaint(hWnd, &ps);
    //         PaintCustomCaption(hWnd, hdc);
    //         EndPaint(hWnd, &ps);
    //     }

    //     fCallDWP = true;
    //     lRet = 0;
    // }

    // Handle the non-client size message.
    if message == WM_NCCALCSIZE && wParam as BOOL == TRUE {
        // Calculate new NCCALCSIZE_PARAMS based on custom NCA inset.
        // NCCALCSIZE_PARAMS *pncsp = reinterpret_cast<NCCALCSIZE_PARAMS*>(lParam);
        let pncsp = &mut *(lParam as *mut NCCALCSIZE_PARAMS);

        pncsp.rgrc[0].left   -= 0;
        pncsp.rgrc[0].top    -= 31;
        pncsp.rgrc[0].right  += 0;
        pncsp.rgrc[0].bottom += 0;

        lRet = 0;

        // No need to pass the message on to the DefWindowProc.
        // fCallDWP = false;
    }

    // Handle hit testing in the NCA if not handled by DwmDefWindowProc.
    if message == WM_NCHITTEST && lRet == 0 {
        lRet = match DefSubclassProc(hWnd, message, wParam, lParam) {
            HTCLIENT => HTCAPTION,
            ret => ret,
        };

        if lRet != HTNOWHERE {
            fCallDWP = false;
        }
    }

    *pfCallDWP = fCallDWP;

    lRet
}
