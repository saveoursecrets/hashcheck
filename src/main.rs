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
use std::{
    fmt, net::SocketAddr, path::PathBuf, str::FromStr, sync::Arc,
    time::SystemTime,
};

struct MetaData {
    last_updated: String,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct HibpService {
    /// Bind address.
    #[clap(short, long, default_value = "0.0.0.0:3342")]
    bind: String,

    /// Read last updated date from this file.
    #[clap(short, long, default_value = "last-updated.txt")]
    last_updated: PathBuf,

    /// Bloom filter data file.
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
    use tracing_subscriber::{
        layer::SubscriberExt, util::SubscriberInitExt,
    };
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    let args = HibpService::parse();
    let addr = args.bind.parse::<SocketAddr>()?;

    let last_updated = std::fs::read_to_string(&args.last_updated)?
        .trim()
        .to_owned();
    let meta_data = MetaData { last_updated };

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
    tracing::info!(
        "finished creating bloom filter in {:#?}",
        now.elapsed()?
    );

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

    let app = Router::new()
        .route("/", get(home).post(check_batch))
        .route("/:hash", get(check_hash))
        .layer(Extension(Arc::new(meta_data)))
        .layer(Extension(Arc::new(bloom)));

    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn home(
    Extension(meta_data): Extension<Arc<MetaData>>,
) -> Json<Value> {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    Json(json!({
        "name": name,
        "version": version,
        "updated": meta_data.last_updated
    }))
}

async fn check_hash(
    Extension(bloom): Extension<Arc<EasyBloom>>,
    Path(hash): Path<PasswordHash>,
) -> Json<Value> {
    // Must be upper case for the check
    let value = hash.to_string();
    let check = bloom.check(&value.into_bytes());
    let value = if check { 1 } else { 0 };
    Json(json!(value))
}

async fn check_batch(
    Extension(bloom): Extension<Arc<EasyBloom>>,
    Json(hashes): Json<Vec<PasswordHash>>,
) -> Json<Value> {
    let checks: Vec<u8> = hashes
        .into_iter()
        .map(|hash| {
            // Must be upper case for the check
            let value = hash.to_string();
            let check = bloom.check(&value.into_bytes());
            if check {
                1
            } else {
                0
            }
        })
        .collect();
    Json(json!(checks))
}
