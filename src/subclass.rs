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
    static mut SUBCLASS_ID: UINT_PTR = 0;
    let data = Box::new(subclass);
    let data = Box::into_raw(data);
    unsafe {
        SetWindowSubclass(h_wnd, Some(subclass_wnd_proc::<S>), SUBCLASS_ID, data as usize);

        let data = &mut *data;
        data.init(h_wnd, SUBCLASS_ID);
        SUBCLASS_ID += 1;
    }
}

unsafe extern "system" fn subclass_wnd_proc<S: WindowSubclass>(
    h_wnd: HWND,
    message: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
    _u_id_subclass: UINT_PTR,
    dw_ref_data: DWORD_PTR,
) -> LRESULT {
    let data = dw_ref_data as *mut S;
    let data = &mut *data;
    data.wnd_proc(h_wnd, message, w_param, l_param)
}
