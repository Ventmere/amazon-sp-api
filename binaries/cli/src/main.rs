use clap::Parser;
use anyhow::Result;

mod exchange_refresh_token;
mod migrate_mws_auth_token;
mod refresh_access_token;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Parser, Debug)]
enum Cmd {
    ExchangeRefreshToken {
        code: String,
    },
    RefreshAccessToken {
        refresh_token: String,
    },
    MigrateMwsAuthToken {
        selling_partner_id: String,
        developer_id: String,
        mws_auth_token: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    match args.cmd {
        Cmd::ExchangeRefreshToken { code } => {
            exchange_refresh_token::run(code).await?;
        }
        Cmd::MigrateMwsAuthToken { selling_partner_id, developer_id, mws_auth_token } => {
            migrate_mws_auth_token::run(selling_partner_id, developer_id, mws_auth_token).await?;
        }
        Cmd::RefreshAccessToken { refresh_token } => {
            refresh_access_token::run(refresh_token).await?;
        }
    }

    Ok(())
}