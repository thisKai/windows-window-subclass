use {
    super::WindowSubclass,
    winapi::{
        shared::{
            basetsd::*,
            minwindef::*,
            windef::*,
        },
        um::commctrl::SetWindowSubclass,
    },
};

pub fn subclass_win32_window<S: WindowSubclass>(h_wnd: HWND, subclass: S) {
    let data: Box<Box<dyn WindowSubclass>> = Box::new(Box::new(subclass));
    let data = Box::into_raw(data);
    unsafe {
        SetWindowSubclass(h_wnd, Some(subclass_wnd_proc), 0, data as usize);
        
        let data = &mut **data;
        data.init(h_wnd);
    }
}

unsafe extern "system" fn subclass_wnd_proc(
    h_wnd: HWND,
    message: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
    _u_id_subclass: UINT_PTR,
    dw_ref_data: DWORD_PTR,
) -> LRESULT {
    let data = dw_ref_data as *mut Box<dyn WindowSubclass>;
    let data = &mut **data;
    data.wnd_proc(h_wnd, message, w_param, l_param)
}
