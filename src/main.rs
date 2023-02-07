use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use book_utils::get_covers;
use epub::doc::EpubDoc;
use std::fs;
use std::io::Write;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/numberland")]
async fn get_book() -> impl Responder {
    get_covers();
    let doc = EpubDoc::new("books/Alex Bellos/Alex's Adventures in Numberland (90)/Alex's Adventures in Numberland - Alex Bellos.epub");
    match doc {
        Ok(mut d) => {
            let title = d.mdata("title").unwrap();
            let cover_data = d.get_cover().unwrap();
            let f = fs::File::create("./covers/numberland.png");
            let mut f = f.unwrap();
            f.write_all(&cover_data).expect("error writing cover");
            // let text = d.
            HttpResponse::Ok().body(title)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(greet).service(get_book))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
