use clap::Parser;
use anyhow::Result;

mod generate;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Parser, Debug)]
enum Cmd {
    /// Generate client crates from swagger definitions
    Generate {
        filter_module: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    match args.cmd {
        Cmd::Generate { filter_module } => {
            generate::run(filter_module.as_deref())?;
        }
    }

    Ok(())
}