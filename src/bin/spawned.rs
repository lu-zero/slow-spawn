fn main() {
    println!("Hello from the spawned!");
    std::thread::sleep(std::time::Duration::from_millis(500));
    println!("Hello from the spawned after sleeping!");
}