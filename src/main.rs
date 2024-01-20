mod binary_search;
mod selection_sort;
mod quick_sort;
mod breadth_first;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use actix_files::NamedFile;
use std::path::PathBuf;

async fn index() -> Result<NamedFile> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[post("/echo")]
async fn echo(f_req_body: String) -> impl Responder {
    HttpResponse::Ok().body(f_req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}