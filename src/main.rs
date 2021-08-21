use bindings::{
    Windows::Win32::Foundation::{
        HWND, RECT,
    },
    Windows::Win32::UI::WindowsAndMessaging::{
        FindWindowW, GetWindowRect,
    },
    Windows::Win32::Graphics::Gdi::{
        SRCCOPY, DIB_RGB_COLORS, BI_RGB, HDC, HBITMAP, 
        HGDIOBJ, BITMAPINFO, BITMAPINFOHEADER,
        CreatedHDC, GetDC, CreateCompatibleDC, SelectObject, 
        BitBlt, ReleaseDC, DeleteDC, DeleteObject, 
        CreateDIBSection,
    },
};
use std::mem;
use std::ptr::null_mut;
use std::path::Path;
use image;

// Capture a window and return its pixel data
fn capture_window(window_title: &str) -> Vec<u8> {
    // Window handle + rect
    let hwnd: HWND = unsafe { FindWindowW(None, window_title) };
    let mut wnd_rect: RECT = unsafe { mem::zeroed() };
    unsafe { GetWindowRect(hwnd, &mut wnd_rect) };
    let width: i32 = wnd_rect.right - wnd_rect.left;
    let height: i32 = wnd_rect.bottom - wnd_rect.top;

    // Device contexts
    let wnd_hdc: HDC = unsafe { GetDC(hwnd) };
    let capture_hdc: CreatedHDC = unsafe { CreateCompatibleDC(wnd_hdc) };

    // Format pixels should be grabbed in
    let bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: -height,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB as u32,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: unsafe { mem::zeroed() },
    };

    // Create bitmap / pointer to raw data
    let mut bits = null_mut();
    let hbitmap: HBITMAP = unsafe { 
        CreateDIBSection(capture_hdc, &bmi, DIB_RGB_COLORS, &mut bits as _, None, 0) 
    };

    // Select new bitmap into captureDC
    let hbitmap_old: HGDIOBJ = unsafe { SelectObject(capture_hdc, hbitmap) };

    // Copy the screen from memory
    unsafe { BitBlt(capture_hdc, 0, 0, width, height, wnd_hdc, 0, 0, SRCCOPY) };

    // Convert raw pointer into Vec
    let size = (width * height * 4) as usize;
    let bits = unsafe { std::slice::from_raw_parts(bits as *mut u8, size).to_owned() };

    // Remove alpha channel values and change from BGR to RGB
    let mut rgb_bits = Vec::<u8>::new();
    for (i, val) in bits.into_iter().enumerate() {
        if (i + 1) % 4 != 0 {
            rgb_bits.push(val);
        }
    }
    for bit in rgb_bits.chunks_exact_mut(3) {
        bit.swap(0, 2);
    }

    // For debugging: Save pixel data to file
    image::save_buffer(&Path::new("frame.png"), &rgb_bits, width as u32, height as u32, image::ColorType::Rgb8)
        .expect("Error saving captured frame");

    // Cleanup
    unsafe { SelectObject(capture_hdc, hbitmap_old) };
    unsafe { DeleteDC(capture_hdc) };
    unsafe { ReleaseDC(hwnd, wnd_hdc) };
    unsafe { DeleteObject(hbitmap) };

    // Return captured bits
    rgb_bits
}

fn main() {
    // test window capturing
    capture_window("NES - Super Mario Bros.");
}
