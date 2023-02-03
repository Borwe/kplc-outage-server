use actix_web::{web, HttpServer, App, HttpResponse};
use serde::{Serialize,Deserialize};
use tokio::sync::RwLock;
use std::sync::Arc;
use std::collections::HashMap;
use kplc_outage_parser::prelude::*;
use anyhow::Result;

type ArcKPLCData = Arc<RwLock<HashMap<String, Arc<KPLCData>>>>;

const KPLC_URL: &str ="https://www.kplc.co.ke/";

#[derive(Serialize, Deserialize)]
struct RequestInfo{
    url: String
}

#[derive(Serialize, Deserialize)]
struct Reply<T>{
    success: bool,
    data: Option<T>,
    message: String,
}

async fn handle_parsing_pdf(json: web::Json<RequestInfo>, 
    data: web::Data<ArcKPLCData>) -> HttpResponse{

    // fail with BadRequest if url doesn't contain kplc.co.ke
    if json.url.contains(KPLC_URL)==false {
        println!("Wrong url passed: {}",json.url);
        let error_website: Reply<u8> = Reply{
            success: false,
            data: None,
            message: format!("Please pass a valid url to a pdf, starting with '{KPLC_URL}'")
        };
        return HttpResponse::BadRequest().json(error_website)
    }

    let kplc_data_respo: Option<Arc<KPLCData>> = {
        let kplc_d_l = data.read().await;
        match kplc_d_l.get(&json.url) {
            Some(x) => Some(x.clone()),
            None => None
        }
    };

    match kplc_data_respo {
        Some(x) => return HttpResponse::Ok().json(Reply::<&KPLCData>{
            data: Some(x.as_ref()),
            success: true,
            message: "success".to_string()
        }),
        None => {
            let mut kplc_d_l = data.write().await;
            println!("Fetching pdf for passing at url: {}",json.url);
            match KPLCClient::new().parse_from_web(&json.url).await{
                Ok(kplc) => {
                    kplc_d_l.insert(json.url.clone(), Arc::new(kplc));
                    HttpResponse::Ok().json(Reply::<&KPLCData>{
                        data: Some(kplc_d_l.get(&json.url).unwrap().as_ref()),
                        message: "success".to_string(),
                        success: true
                    })
                },
                Err(_)=> HttpResponse::BadRequest().json(Reply::<Option<u8>>{
                    data: None,
                    message: format!("There was an error parsing the url: {}",json.url),
                    success: false
                })
            }
        }
    }
    
}

#[tokio::main]
async fn main() -> Result<()>{
    // we use port 7777 by default
    let port: u16 = match std::env::var("PORT") {
        Ok(x) => x.parse().unwrap(),
        Err(_) => 7777
    };


    HttpServer::new(|| {
        // for holding KPLCData results
        // that are linked to a specific
        // website
        let kplc_datas:  ArcKPLCData
                = Arc::new(RwLock::new(HashMap::new()));
        App::new()
            .app_data(web::Data::new(kplc_datas))
            .route("/",web::post().to(handle_parsing_pdf))
    }).bind(("0.0.0.0",port))?.run().await?;
    Ok(())
}
