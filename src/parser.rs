use std::collections::HashMap;
use crate::site_downloader;
use std::process;
pub fn site_parser(url_key: u8) {

    // URL Dictionary
    let mut urls = HashMap::new();
    urls.insert(1, "https://web.facebook.com/login");
    urls.insert(2, "https://accounts.google.com/");

    // Obtiene el URL correspondiente al key

    if let Some(url) = urls.get(&url_key) {
        println!("Cloning URL: {}", url); 
    } else {
        println!("Método no encontrado");
        process::exit(0);
    }

    match url_key{
        1 => site_downloader::facebook("http://127.0.0.1:80/post"),
        2 => println!("Gmail"),
        _ => println!("?")
    } 
    
}
