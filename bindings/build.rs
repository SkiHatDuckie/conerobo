fn main() {
    windows::build! {
        Windows::Win32::Foundation::{
            HWND, RECT
        },
        Windows::Win32::UI::WindowsAndMessaging::{
            FindWindowW, GetWindowRect,
        },
        Windows::Win32::Graphics::Gdi::{
            BI_RGB, HDC, HBITMAP, HGDIOBJ, BITMAPINFO, 
            BITMAPINFOHEADER, CreatedHDC, GetDC, 
            CreateCompatibleDC, SelectObject, BitBlt, 
            ReleaseDC, DeleteDC, DeleteObject, CreateDIBSection,
        },
    }
}
