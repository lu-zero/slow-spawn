fn main() {
    let t = std::time::SystemTime::now();
    let s = std::env::args().nth(1).unwrap();
    let start = std::time::Duration::from_micros(s.parse().unwrap());
    let now = t.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap();
    println!("Hello from the spawned! {:?}", (now - start));
    std::thread::sleep(std::time::Duration::from_millis(500));
    println!(
        "Hello from the spawned after sleeping! {:?}",
        t.elapsed().unwrap()
    );
}
