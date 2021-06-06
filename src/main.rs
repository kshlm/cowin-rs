mod api;
mod client;

use api::States;

#[async_std::main]
async fn main() -> surf::Result<()> {
    let states: States = States::get().await?;
    dbg!(states);
    Ok(())
}
