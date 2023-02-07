use epub::doc::EpubDoc;
use std::fs;
use std::io::Write;

pub async fn get_covers() -> std::result::Result<String, String> {
    let doc = EpubDoc::new("books/Alex Bellos/Alex's Adventures in Numberland (90)/Alex's Adventures in Numberland - Alex Bellos.epub");
    match doc {
        Ok(mut d) => {
            let title = d.mdata("title").unwrap();
            let cover_data = d.get_cover().unwrap();
            let f = fs::File::create("./covers/numberland.png");
            let mut f = f.unwrap();
            f.write_all(&cover_data).expect("error writing cover");
            // let text = d.
            Ok(title)
        }
        _ => Err("oops".to_string()),
    }
}
