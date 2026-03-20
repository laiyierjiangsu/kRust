//! Spawns `ra_tick` from the same directory as this binary (`target/debug` after `cargo build`).
//! Run: `cargo run -p ra_tick_launcher` — then attach LLDB to the **child** PID printed below.

use std::path::PathBuf;
use std::process::{Command, Stdio};

fn ra_tick_executable() -> PathBuf {
    let self_exe = std::env::current_exe().expect("current_exe");
    let dir = self_exe.parent().expect("executable directory");
    let name = if cfg!(target_os = "windows") {
        "ra_tick.exe"
    } else {
        "ra_tick"
    };
    dir.join(name)
}

fn main() {
    let tick = ra_tick_executable();
    if !tick.exists() {
        eprintln!(
            "ra_tick not found at: {}\nBuild both first: cargo build -p ra_tick -p ra_tick_launcher",
            tick.display()
        );
        std::process::exit(1);
    }

    let launcher_pid = std::process::id();
    println!("ra_tick_launcher (parent) pid = {launcher_pid}");
    println!("spawning child: {}", tick.display());

    let mut child = Command::new(&tick)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap_or_else(|e| panic!("failed to spawn ra_tick: {e}"));

    let child_pid = child.id();
    println!("ra_tick (child) pid = {child_pid}  ← attach LLDB to this process (or by executable ra_tick)");

    let status = child.wait().expect("wait on child");
    println!("child exited with: {status}");
}
