use std::ffi::CString;

use libc::*;

// https://doc.rust-lang.org/nomicon/ffi.html

#[link(name = ":libraylib.a")]
unsafe extern "C" {
    fn InitWindow(width: c_int, height: c_int, title: *const c_char);
    fn CloseWindow();
    fn WindowShouldClose() -> bool;
    fn BeginDrawing();
    fn EndDrawing();
    fn ClearBackground(color: c_uint);
    fn DrawText(text: *const c_char, posX: c_int, posY: c_int, fontSize: c_int, color: c_uint);
    fn GetMonitorRefreshRate(monitor: c_int) -> c_int;
    fn GetCurrentMonitor() -> c_int;
    fn SetTargetFPS(fps: c_int);
}
fn main() {
    println!("Hello, world!");
    let title = CString::new("Hello from Rust!").unwrap();
    let text = CString::new("Hello, Raylib from Rust!").unwrap();
    unsafe {
        InitWindow(500, 500, title.as_ptr());
        SetTargetFPS(GetMonitorRefreshRate(GetCurrentMonitor()));
        while !WindowShouldClose() {
            BeginDrawing();
            ClearBackground(0x000000); // Black background
            DrawText(text.as_ptr(), 10, 10, 20, 0xFFFFFFFF); // White text
            EndDrawing();
        }
    }
}
