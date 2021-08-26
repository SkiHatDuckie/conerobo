mod capture;

use capture::capture_screen;

fn main() {
    // Test screen capturing
    capture_screen(100, 100, 300, 300);
}
