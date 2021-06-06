mod api;
mod client;

use color_eyre::eyre::Result;

use api::location::StatesAndDistricts;

#[async_std::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let sd = StatesAndDistricts::get().await?;
    dbg!(sd);

    Ok(())
}
