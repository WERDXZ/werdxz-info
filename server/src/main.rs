mod portfolio;

#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use std::path::PathBuf;

#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>>{
    match NamedFile::open("./static/html/index.html").await {
        Ok(f) => Ok(f),
        Err(_) => Err(NotFound("Page not found".to_string())),
    }
}

#[get("/assets/<path..>")]
async fn assets(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("./static/assets/").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => Err(NotFound("Assets not found".to_string())),
    }
}

#[get("/generated/assets/<path..>")]
async fn generated_assets(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("./generated/assets/").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => Err(NotFound("Assets not found".to_string())),
    }
}

// Finlay start the web sever using the launch macro.
#[launch]
fn rocket() -> _ {
    // You must mount the index route
    rocket::build()
        .mount("/", routes![index, assets, generated_assets])
        .mount("/portfolio", portfolio::routes())
}
