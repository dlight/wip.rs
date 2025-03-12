use inotify::{Inotify, WatchMask};
use std::error::Error;

fn wip(pid: i32) -> Result<(), Box<dyn Error>> {
    println!("Monitoring process with PID: {}", pid);

    // Check if process exists initially
    let proc_path = format!("/proc/{}", pid);
    if !std::path::Path::new(&proc_path).exists() {
        return Err(format!("Process {} does not exist", pid).into());
    }

    // Initialize inotify
    let mut inotify = Inotify::init()?;

    // Watch for DELETE_SELF event on the process directory
    inotify.watches().add(&proc_path, WatchMask::DELETE_SELF)?;

    // Buffer for events
    let mut buffer = [0; 1024];

    // Wait for events
    println!("Waiting for process {} to terminate...", pid);
    let _ = inotify.read_events_blocking(&mut buffer)?;

    println!("Process {} has terminated", pid);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let cmd = &args.get(1).ok_or("Missing command argument")?;

    if *cmd == "wip" {
        let pid = args.get(2).ok_or("Missing PID argument")?.parse::<i32>()?;
        assert!(pid > 0);
        wip(pid)?;
    }

    Ok(())
}
