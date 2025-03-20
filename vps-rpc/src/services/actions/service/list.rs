/// list running service
pub fn list_running_service() {
    // for now only docker is supported
    let cmd = std::process::Command::new("sudo")
        .args([
            "docker",
            "ps",
            "--format",
            "\"{{.Names}}|{{.Status}}|{{.RunningFor}}\"",
        ])
        .output()
        .expect("is docker installed ? sudo right ?");
    let docker_output = String::from_utf8_lossy(&cmd.stdout);
    println!("{docker_output}");
}
