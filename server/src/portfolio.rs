use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use std::path::PathBuf;

const PORTFOLIO: &str = "./generated/portfolio";

#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    match NamedFile::open(&format!("{}/index.html", PORTFOLIO)).await {
        Ok(f) => Ok(f),
        Err(_) => Err(NotFound("Page not found".to_string())),
    }
}

#[get("/<path..>")]
async fn assets(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from(&format!("{}/", PORTFOLIO)).join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => Err(NotFound("Assets not found".to_string())),
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, assets]
}
