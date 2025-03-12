//use inotify::{Inotify, WatchMask};
use std::error::Error;
use waitpid_any::WaitHandle;

fn wip(pid: i32) -> Result<(), Box<dyn Error>> {
    let mut wait_handle = WaitHandle::open(pid)?;
    println!("Waiting for process {} to exit...", pid);
    wait_handle.wait()?;
    println!("Process {} exited", pid);
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
