#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_comparisons)]

use std::sync::Arc;

use anyhow::Result;
use tracing::{error, info};

mod cli;
mod config;
mod utils;

use cli::CliArgs;
use config::Config;

fn generate_registration(args: &CliArgs, compat_mode: bool) -> Result<()> {
    use rand::Rng;
    use std::fs::File;
    use std::io::Write;

    let listen_address = args.listen_address.clone().unwrap_or_else(|| "127.0.0.1".to_string());
    let listen_port = args.listen_port.unwrap_or(28464);

    let mut rng = rand::rng();
    let as_token: String = (0..64).map(|_| rng.sample(rand::distr::Alphanumeric) as char).collect();
    let hs_token: String = (0..64).map(|_| rng.sample(rand::distr::Alphanumeric) as char).collect();

    let mut registration = serde_json::json!({
        "id": "zulipbridge",
        "url": format!("http://{}:{}", listen_address, listen_port),
        "as_token": as_token,
        "hs_token": hs_token,
        "rate_limited": false,
        "sender_localpart": "zulipbridge",
        "namespaces": {
            "users": [{
                "regex": "@zulip_.*",
                "exclusive": true
            }],
            "aliases": [],
            "rooms": []
        }
    });

    if compat_mode {
        registration["namespaces"]["users"].as_array_mut().unwrap().push(serde_json::json!({
            "regex": "@zulipbridge:.*",
            "exclusive": true
        }));
    }

    let output_path = &args.config;
    if std::path::Path::new(output_path).exists() {
        anyhow::bail!("Registration file already exists, not overwriting.");
    }

    let yaml_content = serde_yaml::to_string(&registration)?;
    let mut file = File::create(output_path)?;
    file.write_all(yaml_content.as_bytes())?;

    info!("Registration file generated and saved to {}", output_path);
    Ok(())
}

async fn run_bridge(args: &CliArgs) -> Result<()> {
    let config = Arc::new(Config::load(&args.config)?);
    config.validate()?;

    info!("matrix-zulip bridge starting up");
    info!("Connecting to homeserver at {}", config.bridge.homeserver_url);

    // TODO: Initialize database manager
    // TODO: Initialize Matrix appservice
    // TODO: Initialize Zulip client
    // TODO: Initialize bridge core
    // TODO: Start web server

    info!("matrix-zulip bridge is running");

    tokio::signal::ctrl_c().await?;
    info!("received Ctrl+C, beginning shutdown");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse_args();

    if args.verbose {
        utils::logging::init_tracing_with_debug();
    } else {
        utils::logging::init_tracing();
    }

    if args.generate {
        generate_registration(&args, false)?;
        return Ok(());
    }

    if args.generate_compat {
        generate_registration(&args, true)?;
        return Ok(());
    }

    if args.reset {
        info!("Reset mode not yet implemented");
        return Ok(());
    }

    run_bridge(&args).await
}
