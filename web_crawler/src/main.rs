use std::env;
use url::Url;

mod crawl;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("{}", args[1]);
        if let Ok(u) = Url::parse(&args[1]) {
            let mut c = crawl::Crawler::new(u.to_string());
            c.crawl();
        } else {
            eprintln!("Error parsing url!");
        }
    }
}
