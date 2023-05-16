fn main() {
    let t = std::time::SystemTime::now();
    let s = format!(
        "{}",
        t.duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros()
    );
    println!("Hello before spawning! (status)");

    std::process::Command::new("./target/release/spawned")
        .arg(s)
        .status()
        .unwrap();

    println!("Hello after spawning! {:?}", t.elapsed().unwrap());

    let t = std::time::SystemTime::now();
    let s = format!(
        "{}",
        t.duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros()
    );

    println!("Hello before spawning! (status + explicit inherit)");

    std::process::Command::new("./target/release/spawned")
        .arg(s)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .unwrap();

    println!("Hello after spawning! {:?}", t.elapsed().unwrap());
}
