//Import the rocket macros
#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use std::path::PathBuf;

#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>>{
    match NamedFile::open("./static/html/index.html").await {
        Ok(f) => Ok(f),
        Err(_) => Err(NotFound("index.html".to_string())),
    }
}

#[get("/image/<path..>")]
async fn image(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("./static/assets/image/").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => Err(NotFound("image".to_string())),
    }
}

#[get("/style/<path..>")]
async fn style(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("./static/assets/style/").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => Err(NotFound("style".to_string())),
    }
}

// Finlay start the web sever using the launch macro.
#[launch]
fn rocket() -> _ {
    // You must mount the index route
    rocket::build()
        .mount("/", routes![index, image, style])
}
