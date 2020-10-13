#[cfg(debug_assertions)]
pub struct Timer {
    time: std::time::Instant,
}

#[cfg(debug_assertions)]
impl Drop for Timer {
    fn drop(&mut self) {
        gdnative::godot_print!("Elapsed time: {}", self.time.elapsed().as_micros());
    }
}

#[cfg(debug_assertions)]
pub fn timer() -> Timer {
    Timer {
        time: std::time::Instant::now(),
    }
}

#[cfg(not(debug_assertions))]
pub fn timer() {}
