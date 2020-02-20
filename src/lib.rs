#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::subclass_window;

#[cfg(all(windows, feature = "winit"))]
pub fn subclass_winit_window(window: &winit::window::Window) {
    use winit::platform::windows::WindowExtWindows;

    subclass_window(window.hwnd() as _);
}
#[cfg(all(not(windows), feature = "winit"))]
pub fn subclass_winit_window(window: &winit::window::Window) {}
