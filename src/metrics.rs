#[cfg(windows)]
use winapi::{
    shared::{
        ntdef::NULL,
        minwindef::*,
        windef::*,
    },
    um::{
        winuser::*,
        uxtheme::MARGINS,
    },
};

#[derive(Debug, Default, Copy, Clone)]
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
    #[cfg(windows)]
    pub(crate) fn winapi(&self) -> MARGINS {
        MARGINS {
            cxLeftWidth: self.left,
            cxRightWidth: self.right,
            cyBottomHeight: self.bottom,
            cyTopHeight: self.top,
        }
    }
}

pub fn window_frame_metrics() -> Result<WindowFrameMetrics, &'static str> {
    #[cfg(not(windows))]
    {
        Err("not windows")
    }
    #[cfg(windows)]
    {
        let (border, titlebar) = unsafe {
            let mut border = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            AdjustWindowRectEx(&mut border, WS_OVERLAPPEDWINDOW & !WS_CAPTION, FALSE, NULL as _);

            let mut with_titlebar = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            AdjustWindowRectEx(&mut with_titlebar, WS_OVERLAPPEDWINDOW, FALSE, NULL as _);
            (border, with_titlebar.top)
        };

        Ok(WindowFrameMetrics {
            titlebar: -titlebar,
            border: Margins {
                left: -border.left,
                top: -border.top,
                right: border.right,
                bottom: border.bottom,
            },
        })
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct WindowFrameMetrics {
    pub titlebar: i32,
    pub border: Margins,
}