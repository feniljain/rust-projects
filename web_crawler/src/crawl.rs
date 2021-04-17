use reqwest;
use select;
use url::Url;

pub struct Crawler {
    url: String,
    active_links: i32,
    inactive_links: i32,
}

impl Crawler {
    pub fn new(u: String) -> Crawler {
        Crawler {
            url: u,
            active_links: 0,
            inactive_links: 0,
        }
    }

    pub fn crawl(&mut self) {
        let resp = reqwest::blocking::get(self.url.as_str()).unwrap();
        //println!("{:#?}", resp);
        match select::document::Document::from_read(resp) {
            Ok(r) => {
                r.find(select::predicate::Name("a"))
                    .filter_map(|n| n.attr("href"))
                    .filter_map(|x| match Url::parse(&x) {
                        Ok(url) => {
                            let urlc = url.clone();
                            let url_str = urlc.as_str();
                            let resp = reqwest::blocking::get(url).expect(url_str);
                            if resp.status().is_success() {
                                self.active_links += 1;
                            } else {
                                self.inactive_links += 1;
                            }
                            Some(x)
                        }
                        Err(_) => None,
                    })
                    .for_each(|x| println!("{}", x));
            }
            Err(err) => eprintln!("{}", err),
        }

        println!(
            "Active links: {}, Inactive links: {}",
            self.active_links, self.inactive_links
        );

        //match reqwest::get(self.url.as_str()).await {
        //    Ok(resp) => match
        //        document::Document::from_read(resp) {
        //        Ok(r) => {
        //            r.find(select::Name("a"))
        //                .filter_map(|n| n.attr("href"))
        //                .for_each(|x| println!("{}", x));
        //        }
        //        Err(err) => eprintln!(err),
        //    },
        //    Err(err) => eprintln!("{}", err),
        //}

        //match resp
        //    .json::<HashMap<String, String>>().await {
        //    Ok(res) => println!("{:#?}", res),
        //    Err(err) => println!("{}", err),
        //},

        //let client = Client::new();
        //let s = self.url.parse().unwrap();
        //let uri = Uri::from(s);
        //println!("Here!");
        //match client.get(uri).await {
        //    Ok(resp) => {
        //        //println!("{}", respBody);
        //        //let body = resp.body();
        //        let status = resp.status();
        //        println!("{}", status);

        //        if let Ok(buf) = hyper::body::to_bytes(resp).await {
        //            println!("{:?}", buf);
        //        } else {
        //            eprintln!("Error converting to bytes");
        //        }
        //    }
        //    Err(err) => eprintln!("{}", err),
        //}
    }
}
