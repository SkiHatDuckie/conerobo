mod capture;

use capture::capture_screen;
use std::time::Instant;

fn main() {
    // Test screen capturing
    let mut total_time: f32 = 0.0;
    for _ in 1..=120 {
        let loop_time = Instant::now();
        capture_screen(0, 0, 1920, 1080);

        // Add loop time to total
        total_time += loop_time.elapsed().as_secs_f32()
    }
    println!("Total time: {}", total_time)
}
