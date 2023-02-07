use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use epub::doc::EpubDoc;
use std::fs;
use std::io::Write;
use walkdir::WalkDir;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/")]
async fn index() -> impl Responder {
    let mut body = "Books:\n".to_owned();

    for entry in WalkDir::new("./books") {
        let mut path = entry.unwrap().path().to_str().unwrap().to_owned();
        path.push_str("\n");
        body.push_str(&path);
    }

    HttpResponse::Ok().body(body)
}

#[get("/numberland")]
async fn get_book() -> impl Responder {
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
