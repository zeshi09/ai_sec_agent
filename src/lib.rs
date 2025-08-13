pub mod crawler;

pub fn start_scan(target: &str) {
    println!("[core] start_scan -> {target}")
    crawler::run(target);
}
