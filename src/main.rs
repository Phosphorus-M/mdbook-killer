use anyhow::Result;

use mdbook_killer::cli::get_cli;
use mdbook_killer::models::Config;

#[tokio::main]
pub async fn main() -> Result<()> {
    let cli = get_cli();

    cli.commands.execute().await?;
    Ok(())
}
