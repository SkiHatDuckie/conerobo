use bindings::{
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

// Capture screen and return its pixel data
pub fn capture_screen(x: i32, y: i32, w: i32, h: i32) -> Vec<u8> {
    // Device contexts
    let desktop_hdc: HDC = unsafe { GetDC(None) };
    let capture_hdc: CreatedHDC = unsafe { CreateCompatibleDC(desktop_hdc) };

    // Format pixels should be grabbed in
    let bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: w,
            biHeight: -h,
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
    unsafe { BitBlt(capture_hdc, 0, 0, w, h, desktop_hdc, x, y, SRCCOPY) };

    // Convert raw pointer into Vec
    let size = (w * h * 4) as usize;
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

    // Cleanup
    unsafe { SelectObject(capture_hdc, hbitmap_old) };
    unsafe { DeleteDC(capture_hdc) };
    unsafe { ReleaseDC(None, desktop_hdc) };
    unsafe { DeleteObject(hbitmap) };

    // Return captured bits
    rgb_bits
}
