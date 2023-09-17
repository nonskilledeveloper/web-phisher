use std::io;
mod menus;
mod parser;
mod site_downloader;
mod html_mod;
mod server;

fn main() {

    menus::welcome1();
    menus::clonto();

    let mut site_to_clone = String::new();
    io::stdin()
        .read_line(&mut site_to_clone)
        .expect("Error");

    let site_to_clone: u8 = site_to_clone.trim().parse().expect("e");

    parser::site_parser(site_to_clone);
    
}
