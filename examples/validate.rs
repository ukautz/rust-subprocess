use std::time::Instant;
use subprocess::{Exec, Redirection};

fn main() -> subprocess::Result<()> {
    let start = Instant::now();

    // Some command that backgrounds
    let command = "sleep 4 1>/dev/null 2>/dev/null &";

    let exec = Exec::shell(command).stdout(Redirection::Pipe).stderr(Redirection::Pipe).stdin(Redirection::None);
    let mut process = exec.detached().popen()?;
    let mut comm = process.communicate_start(None);

    let (stdout, stderr) = comm.read()?;
    let exit_status = process.wait()?;
    drop(stdout);
    drop(stderr);
    println!("Exit status: {exit_status:?}");
    println!("Duration: {:?}", start.elapsed());

    Ok(())
}
