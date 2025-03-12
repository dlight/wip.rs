//use inotify::{Inotify, WatchMask};
use std::error::Error;
use waitpid_any::WaitHandle;

fn wip(pids: Vec<i32>) -> Result<(), Box<dyn Error>> {
    let mut handles = pids
        .iter()
        .map(|pid| WaitHandle::open(*pid))
        .collect::<Result<Vec<_>, _>>()?;

    println!("Waiting for process {:?} to exit...", pids);

    let exited_pid;

    'out: loop {
        for (i, handle) in handles.iter_mut().enumerate() {
            if let Some(()) = handle.wait_timeout(std::time::Duration::from_secs(0))? {
                exited_pid = pids[i];
                break 'out;
            }
        }
    }

    println!("Process {:?} exited", exited_pid);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let cmd = &args.get(1).ok_or("Missing command argument")?;

    if *cmd == "wip" {
        let pids = args
            .iter()
            .skip(2)
            .map(|s| {
                let n = s.parse::<i32>().unwrap();
                assert!(n > 0);
                n
            })
            .collect::<Vec<i32>>();

        wip(pids)?;

        return Ok(());
    }

    Ok(())
}
