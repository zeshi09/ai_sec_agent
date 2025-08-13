pub mod web_crawler;

pub fn run(target: &str) {
    println!("[crawler] run -> {target}");
    web_crawler::crawl(target);
}
