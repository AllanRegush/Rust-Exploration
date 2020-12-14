use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::{HBRUSH, HICON, HMENU, HWND, RECT};
use winapi::um::winuser::*;

struct Window {
    hwnd: HWND,
}

static mut isRunning: bool = false;

unsafe extern "system" fn window_proc(
    h_wnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM
) -> LRESULT {
    let mut result = 0;
    match msg {
        WM_DESTROY => {
            isRunning = false;
            PostQuitMessage(0);
        }
        WM_PAINT => {
            
            let mut paint: PAINTSTRUCT = std::mem::zeroed();
            let mut rect: RECT = std::mem::zeroed();
            let mut myString: Vec<u16> = OsStr::new("Hello, World").encode_wide().collect();
            myString.push(0);
            let DeviceContext = BeginPaint(h_wnd, &mut paint);
            GetClientRect(h_wnd, &mut rect);
            DrawTextW(DeviceContext, myString.as_ptr() , -1,
             &mut rect, DT_SINGLELINE | DT_CENTER | DT_VCENTER);
            EndPaint(h_wnd, &paint);
            

        }
        _ => {
            result = DefWindowProcW(h_wnd,msg, w_param,l_param)
        }
    }
    result
}

fn create_window() -> Result<Window, ()> {
    unsafe {
        let mut window_class_name: Vec<u16> = OsStr::new("HelloWorld")
                                                    .encode_wide()
                                                    .collect();
        window_class_name.push(0);

        let window_class = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: 0 as HINSTANCE,
            hIcon: 0 as HICON,
            hCursor: 0 as HICON,
            hbrBackground: 16 as HBRUSH,
            lpszMenuName: 0 as LPCWSTR,
            lpszClassName: window_class_name.as_ptr(),
        };

        let error = RegisterClassW(&window_class);
    
        assert!(error != 0, "failed to register window class");

        let h_wnd_window = CreateWindowExW(
            0,
            window_class_name.as_ptr(),
            0 as LPCWSTR,
            WS_OVERLAPPED | WS_MINIMIZEBOX | WS_SYSMENU,
            0,
            0,
            400,
            400,
            0 as HWND,
            0 as HMENU,
            0 as HINSTANCE,
            std::ptr::null_mut(),
        );

        assert!(h_wnd_window != (0 as HWND), "failed to open the window");

        ShowWindow(h_wnd_window, SW_SHOW);
        isRunning = true;

        Ok(Window { hwnd: h_wnd_window })
    }

}

fn main() {
    let win32_window: Window = create_window().unwrap();

    unsafe {
        while isRunning {
            let mut msg: MSG = std::mem::zeroed();

            if PeekMessageA(&mut msg, win32_window.hwnd, 0, 0, PM_REMOVE) > 0 {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
        }
    }
}
