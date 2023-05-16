fn main() {
    println!("Hello before spawning!");

    std::process::Command::new("./target/release/spawned")
        .arg("4")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .unwrap();

    println!("Hello after spawning!");
}
