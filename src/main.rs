use actix_web::{web, HttpServer, App, Responder, HttpResponse};
use serde::{Serialize,Deserialize};
use anyhow::Result;

const KPLC_URL: &str ="https://kplc.co.ke/";

#[derive(Serialize, Deserialize)]
struct RequestInfo{
    url: String
}

#[derive(Serialize, Deserialize)]
struct Reply{
    success: String,
    message: String
}

async fn handle_parsing_pdf(json: web::Json<RequestInfo>) -> impl Responder{
    if !json.url.starts_with(KPLC_URL) {
        let error_website = Reply{
            success: "false".to_string(),
            message: format!("Please pass a valid url, starting with '{KPLC_URL}'")
        };
        return HttpResponse::BadRequest().json(error_website)
    }
    HttpResponse::Ok().body("YOYO!\n")
}

#[actix_web::main]
async fn main() -> Result<()>{
    // we use port 7777 by default
    let port: u16 = match std::env::var("PORT") {
        Ok(x) => x.parse().unwrap(),
        Err(_) => 7777
    };

    HttpServer::new(||{
        App::new().route("/",web::post().to(handle_parsing_pdf))
    }).bind(("0.0.0.0",port))?.run().await?;
    Ok(())
}
