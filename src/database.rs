use actix_web::web::Data;
use deadpool_postgres::{Client, Pool};
use tokio_postgres::Error;

pub async fn get_nft(
    db_pool: Data<Pool>,
    network: String,
    contract_address: String,
) -> Result<(String, String), Error> {
    let client: Client = db_pool.get().await.unwrap();
    let row = client
      .query_one(
          "SELECT name, discord_url FROM nfts WHERE network = $1 and LOWER(contract_address) = $2 LIMIT 1",
          &[&network, &contract_address.to_lowercase()],
      )
      .await.unwrap();
    let name: String = row.get(0);
    let discord_url: String = row.get(1);
    Ok((name, discord_url))
}

pub async fn get_address(
    db_pool: Data<Pool>,
    network: String,
    from_address: String,
    to_address: String,
) -> Result<(String, String), Error> {
    let client: Client = db_pool.get().await.unwrap();
    let row = client
    .query_one(
        "SELECT name, discord_url FROM addresses WHERE network = $1 and (LOWER(address) = $2 or LOWER(address) = $3) LIMIT 1",
        &[&network, &from_address.to_lowercase(), &to_address.to_lowercase()],
    )
    .await.unwrap();
    let name: String = row.get(0);
    let discord_url: String = row.get(1);
    Ok((name, discord_url))
}
