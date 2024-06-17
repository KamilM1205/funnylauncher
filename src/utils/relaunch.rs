use std::env;

pub fn relaunch() -> Result<(), Box<dyn std::error::Error>> {
    let exe_file = match env::current_exe() {
        Ok(f) => Ok(f),
        Err(e) => Err(e),
    }?;

    std::process::Command::new(&exe_file)
        .stdout(std::process::Stdio::null()) // It's need to launch launcher as proccess, not
        // subproccess
        .stderr(std::process::Stdio::null());

    Ok(())
}
