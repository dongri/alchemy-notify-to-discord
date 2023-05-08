use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use tokio_postgres::NoTls;

mod database;
mod discord;

//================ Address ===================

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookAddress {
    pub webhook_id: String,
    pub id: String,
    pub created_at: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub event: EventAddress,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventAddress {
    pub network: String,
    pub activity: Vec<ActivityAddress>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityAddress {
    pub from_address: String,
    pub to_address: String,
    pub block_num: String,
    pub hash: String,
    pub value: Option<f64>,
    pub asset: Option<String>,
    pub category: String,
    pub raw_contract: RawContract,
    pub log: Option<LogAddress>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawContract {
    #[serde(rename = "rawValue")]
    pub raw_value: String,
    pub address: Option<String>,
    pub decimals: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogAddress {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    pub block_number: String,
    pub transaction_hash: String,
    pub transaction_index: String,
    pub block_hash: String,
    pub log_index: String,
    pub removed: bool,
}

//================ NFT ===================

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookNFT {
    pub webhook_id: String,
    pub id: String,
    pub created_at: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub event: EventNFT,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventNFT {
    pub network: String,
    pub activity: Vec<ActivityNFT>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNFT {
    pub from_address: String,
    pub to_address: String,
    pub contract_address: String,
    pub block_num: String,
    pub hash: String,
    pub erc721_token_id: String,
    pub category: String,
    pub log: Log,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    pub block_number: String,
    pub transaction_hash: String,
    pub transaction_index: String,
    pub block_hash: String,
    pub log_index: String,
    pub removed: bool,
}

#[post("/address")]
async fn address(db_pool: web::Data<Pool>, req_body: String) -> impl Responder {
    println!("req_body: {}", req_body);

    let result: Result<WebhookAddress, serde_json::Error> = serde_json::from_str(&req_body);
    match result {
        Ok(webhook) => {
            println!("webhook_id: {:?}", webhook.webhook_id);
            if webhook.event.activity.len() == 0 {
                discord::post_error("Activity is Zero".to_string()).await;
                return HttpResponse::Ok().body(req_body);
            }
            let activity = webhook.event.activity[0].clone();
            discord::post_event_address(
                db_pool,
                webhook.event.network.clone(),
                activity.from_address.clone(),
                activity.to_address.clone(),
                activity.asset.unwrap_or("".to_string()),
                activity.value.unwrap_or(0.0),
                activity.category.clone(),
                activity.hash.clone(),
            )
            .await;
            HttpResponse::Ok().body(req_body)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            discord::post_error(e.to_string()).await;
            HttpResponse::Ok().body(req_body)
        }
    }
}

#[post("/nft")]
async fn nft(db_pool: web::Data<Pool>, req_body: String) -> impl Responder {
    println!("req_body: {}", req_body);

    let result: Result<WebhookNFT, serde_json::Error> = serde_json::from_str(&req_body);
    match result {
        Ok(webhook) => {
            println!("webhook_id: {:?}", webhook.webhook_id);
            if webhook.event.activity.len() == 0 {
                discord::post_error("Activity is Zero".to_string()).await;
                return HttpResponse::Ok().body(req_body);
            }
            let activity = webhook.event.activity[0].clone();
            discord::post_event_nft(
                db_pool,
                webhook.event.network.clone(),
                activity.contract_address.clone(),
                activity.from_address.clone(),
                activity.to_address.clone(),
                activity.log.transaction_hash.clone(),
            )
            .await;
            HttpResponse::Ok().body(req_body)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            discord::post_error(e.to_string()).await;
            HttpResponse::Ok().body(req_body)
        }
    }
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("I am alive!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    dotenv().ok();

    let mut pg_config = tokio_postgres::Config::new();
    pg_config.host(env::var("DATABASE_HOST").unwrap().as_str());
    pg_config.port(env::var("DATABASE_PORT").unwrap().parse::<u16>().unwrap());
    pg_config.user(env::var("DATABASE_USER").unwrap().as_str());
    pg_config.password(env::var("DATABASE_PASSWORD").unwrap().as_str());
    pg_config.dbname(env::var("DATABASE_NAME").unwrap().as_str());
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    let pool = Pool::builder(mgr).max_size(16).build().unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(address)
            .service(nft)
            .service(index)
    })
    .bind(("0.0.0.0", 50018))?
    .run()
    .await
}
