pub mod web_crawler;

pub fn run(target: &str) {
    web_crawler::crawl(target);
}
