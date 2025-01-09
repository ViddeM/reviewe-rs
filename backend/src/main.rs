#![forbid(unsafe_code)]

use eyre::Context;

#[rocket::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install().wrap_err("Failed to install color eyre")?;
    dotenvy::dotenv().ok();

    Ok(())
}
