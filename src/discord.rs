use actix_web::web::Data;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Thumbnail {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Field {
    name: String,
    value: String,
    inline: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Embed {
    title: String,
    url: String,
    thumbnail: Thumbnail,
    fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Discord {
    content: String,
    embeds: Vec<Embed>,
}

pub async fn post_event_address(
    db_pool: Data<Pool>,
    network: String,
    from_address: String,
    to_address: String,
    asset: String,
    value: f64,
    category: String,
    tx_hash: String,
) {
    let db_address = super::database::get_address(
        db_pool,
        network.clone(),
        from_address.clone(),
        to_address.clone(),
    )
    .await;

    let (name, discord_url) = match db_address {
        Ok(db_address) => db_address,
        Err(_) => (
            "None".to_string(),
            env::var(env::var("ERROR_WEBHOOK_URL").unwrap().as_str()).unwrap(),
        ),
    };

    let embed = Embed {
        title: format!("Transaction URL"),
        url: scan_url(network.clone(), tx_hash),
        thumbnail: Thumbnail {
            url: "".to_string(),
        },
        fields: vec![
            Field {
                name: "Name".to_string(),
                value: name,
                inline: false,
            },
            Field {
                name: "Network".to_string(),
                value: network,
                inline: false,
            },
            Field {
                name: "From".to_string(),
                value: from_address,
                inline: false,
            },
            Field {
                name: "To".to_string(),
                value: to_address,
                inline: false,
            },
            Field {
                name: "Value".to_string(),
                value: format!("{:?}", value),
                inline: false,
            },
            Field {
                name: "Asset".to_string(),
                value: format!("{:?}", asset),
                inline: false,
            },
            Field {
                name: "Category".to_string(),
                value: category,
                inline: false,
            },
        ],
    };

    let discord = Discord {
        content: format!("Address Activity"),
        embeds: vec![embed],
    };

    let client = reqwest::Client::new();
    let res = client.post(discord_url).json(&discord).send().await;
    println!("{}", "send to discord");
    match res {
        Ok(res) => {
            println!("{:?}", res.status());
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

pub async fn post_event_nft(
    db_pool: Data<Pool>,
    network: String,
    contract_address: String,
    from_address: String,
    to_address: String,
    tx_hash: String,
) {
    let db_nft = super::database::get_nft(db_pool, network.clone(), contract_address.clone()).await;
    let (name, discord_url) = match db_nft {
        Ok(db_nft) => db_nft,
        Err(_) => (
            "None".to_string(),
            env::var(env::var("ERROR_WEBHOOK_URL").unwrap().as_str()).unwrap(),
        ),
    };

    let embed = Embed {
        title: format!("Transaction URL"),
        url: scan_url(network.clone(), tx_hash),
        thumbnail: Thumbnail {
            url: "".to_string(),
        },
        fields: vec![
            Field {
                name: "NFT Name".to_string(),
                value: name,
                inline: false,
            },
            Field {
                name: "Network".to_string(),
                value: network,
                inline: false,
            },
            Field {
                name: "ContractAddress".to_string(),
                value: contract_address,
                inline: false,
            },
            Field {
                name: "FromAddress".to_string(),
                value: from_address,
                inline: false,
            },
            Field {
                name: "ToAddress".to_string(),
                value: to_address,
                inline: false,
            },
        ],
    };

    let discord = Discord {
        content: format!("NFT Activity"),
        embeds: vec![embed],
    };

    let client = reqwest::Client::new();
    let res = client.post(discord_url).json(&discord).send().await;
    println!("{}", "send to discord");
    match res {
        Ok(res) => {
            println!("{:?}", res.status());
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

pub async fn post_error(message: String) {
    let mut map = HashMap::new();
    map.insert("content", format!("Error: {}", message));

    let client = reqwest::Client::new();
    let res = client
        .post(env::var("ERROR_WEBHOOK_URL").unwrap().as_str())
        .json(&map)
        .send()
        .await;
    println!("{:?}", res.unwrap().status());
}

pub fn scan_url(network: String, tx_hash: String) -> String {
    let etherscan;
    match network.as_str() {
        "ETH_GOERLI" => {
            etherscan = "https://goerli.etherscan.io/tx";
        }
        "ETH_MAINNET" => {
            etherscan = "https://etherscan.io/tx";
        }
        "MATIC_MUMBAI" => {
            etherscan = "https://mumbai.polygonscan.com/tx";
        }
        "MATIC_MAINNET" => {
            etherscan = "https://polygonscan.com/tx";
        }
        _ => {
            etherscan = "https://etherscan.io/tx";
        }
    }

    format!("{}/{}", etherscan, tx_hash)
}
