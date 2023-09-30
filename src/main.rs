use anyhow::Result;
use axum::{
    extract::{Extension, Path},
    routing::get,
    Json, Router,
};
use clap::Parser;
use easypwned_bloom::bloom::{bloom_get, EasyBloom};
use human_bytes::human_bytes;
use serde::Deserialize;
use serde_json::{json, Value};
use std::{fmt, net::SocketAddr, str::FromStr, sync::Arc, time::SystemTime};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct HibpService {
    #[clap(short, long, default_value = "0.0.0.0:3342")]
    bind: String,

    #[clap(default_value = "hibp.bloom")]
    file: String,
}

/// SHA-1 password hash.
#[derive(Deserialize)]
pub struct PasswordHash(#[serde(with = "hex::serde")] Vec<u8>);

impl fmt::Display for PasswordHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0).to_uppercase())
    }
}

impl FromStr for PasswordHash {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let digest = hex::decode(s)?;
        Ok(PasswordHash(digest))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    let args = HibpService::parse();
    let addr = args.bind.parse::<SocketAddr>()?;

    let meta = std::fs::metadata(&args.file)?;
    tracing::info!(
        "reading bloom filter file {} ({})",
        &args.file,
        human_bytes(meta.len() as f64)
    );
    let now = SystemTime::now();
    let bloom = match bloom_get(&args.file) {
        Ok(b) => b,
        Err(e) => {
            panic!("could not read bloom filter data {}", e);
        }
    };
    tracing::info!(
        "finished reading bloom filter file in {:#?}",
        now.elapsed()?
    );

    tracing::info!("creating bloom filter");
    let now = SystemTime::now();
    let bloom = bloom.to_bloom();
    tracing::info!("finished creating bloom filter in {:#?}", now.elapsed()?);

    let checks = vec![
        "0000000CAEF405439D57847A8657218C618160B2",
        "0000000CAEF405439D57847A8657218C618160BX",
    ];

    for check in checks {
        tracing::info!(
            "check: {} -> {:?}",
            check,
            bloom.check(&check.as_bytes().to_vec())
        );
    }

    let bloom_ext = Arc::new(bloom);
    let app = Router::new()
        .route("/hash/:hash", get(handler_hash))
        .layer(Extension(bloom_ext));

    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn handler_hash(
    Extension(bloom): Extension<Arc<EasyBloom>>,
    Path(hash): Path<PasswordHash>,
) -> Json<Value> {
    // Must be upper case for the check
    let value = hash.to_string();
    let check = bloom.check(&value.as_bytes().to_vec());
    Json(json!({"secure": !check}))
}
