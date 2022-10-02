use clap::Parser;
use anyhow::Result;

mod auth;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Parser, Debug)]
enum Cmd {
    Auth {
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
        Cmd::Auth { selling_partner_id, developer_id, mws_auth_token } => {
            auth::run(selling_partner_id, developer_id, mws_auth_token).await?;
        }
    }

    Ok(())
}