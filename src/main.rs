use rocket::{
    data::ToByteUnit, fs::NamedFile, http::uri::Absolute, response::status::NotFound, Data,
};
use std::path::Path;

mod past_id;
use past_id::PasteId;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, retrieve, upload])
}

#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/index.html");
    NamedFile::open(&path)
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<NamedFile> {
    NamedFile::open(id.file_path()).await.ok()
}

const ID_LENGTH: usize = 8;
const HOST: Absolute<'static> = uri!("http://localhost:8000");

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    paste
        .open(128.kilobytes())
        .into_file(id.file_path())
        .await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}
