#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::response::status;

mod app;
use app::manifest_parser::parser;

#[derive(FromForm)]
struct Upload<'r> {
    apk: TempFile<'r>,
}

#[post("/", data = "<upload>")]
async fn upload(upload: Form<Upload<'_>>) -> status::Custom<String> {
    let path = upload.apk.path().unwrap().to_path_buf();
    if let Some(info) = parser::parse(&path).await {
        let string_json = serde_json::to_string(&info);
        if string_json.is_ok() {
            return status::Custom(Status::Accepted, string_json.unwrap());
        }
    }
    status::Custom(Status::UnprocessableEntity, "oops!".to_owned())
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![upload])
        .launch()
        .await
        .unwrap();

    Ok(())
}
