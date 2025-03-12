use nix::sys::prctl::set_pdeathsig;
use nix::sys::signal::Signal;
use nix::unistd::{getpid, getppid};
use signal_hook::{consts::SIGUSR1, flag};

use std::{
    error::Error,
    sync::Arc,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

fn wip() -> Result<(), Box<dyn Error>> {
    let pid = getpid();
    let ppid = getppid();
    println!("Process {} monitoring parent {}", pid, ppid);

    // Create an atomic flag that will be set when we receive the signal
    let parent_died = Arc::new(AtomicBool::new(false));

    // Set up the signal handler using signal_hook's flag API
    // This registers a handler that sets our atomic flag when SIGUSR1 is received
    flag::register(SIGUSR1, parent_died.clone())?;

    // Tell kernel to send SIGUSR1 when parent dies
    set_pdeathsig(Signal::SIGUSR1)?;

    // Check if parent is already PID 1 (init)
    if ppid.as_raw() == 1 {
        println!("Parent is already init (PID 1)");
        return Ok(());
    }

    println!("Monitoring parent {}...", ppid);

    // Single loop that checks both the signal flag and the parent PID
    while !parent_died.load(Ordering::Relaxed) {
        // Check if parent has changed to PID 1
        if getppid().as_raw() == 1 || getppid() != ppid {
            println!("Parent changed to {}", getppid());
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }

    // We get here either via the signal or the PID check
    println!("Parent died - performing cleanup...");

    // Do cleanup work here

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = std::env::args().nth(1).ok_or("No command provided")?;

    if cmd == "wip" {
        wip()?;
    }

    Ok(())
}
