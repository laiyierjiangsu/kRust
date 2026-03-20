//! Prints a tick message every interval. Run under a terminal, then attach LLDB from the IDE.

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let pid = std::process::id();
    println!("ra_tick started (pid = {pid}). Build: debug symbols on. Ctrl+C to exit.");
    let _ = io::stdout().flush();

    let mut tick: u64 = 0;
    loop {
        tick = tick.wrapping_add(1);
        // Breakpoint-friendly line: pause here after attach.
        println!("[ra_tick] tick {tick}");
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_secs(2));
    }
}
