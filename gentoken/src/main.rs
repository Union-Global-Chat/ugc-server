use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use clap::{Parser, Subcommand};
use db::token::{add_token, del_token};
use getrandom::getrandom;
use sqlx::MySqlPool;
use std::env;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Gen {
        #[arg(short)]
        bot_id: i64,
    },
    Del {
        #[arg(short)]
        bot_id: i64,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
    let cli = Cli::parse();
    match cli.command {
        Commands::Gen { bot_id } => {
            let mut buf: [u8; 32] = [0u8; 32];
            getrandom(&mut buf)?;
            let token = BASE64_URL_SAFE_NO_PAD.encode(buf);
            add_token(&pool, bot_id, token.clone()).await?;
            println!("Token was generate: {}", token);
        }
        Commands::Del { bot_id } => {
            del_token(&pool, bot_id).await?;
            println!("Deleted");
        }
    }
    Ok(())
}
