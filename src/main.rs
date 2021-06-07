use clap::Clap;
use color_eyre::eyre::Result;

use cowin_rs::api::location::StatesAndDistricts;

#[async_std::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let app = App::parse();
    match app.subcmd {
        SubCommand::DistrictId(d) => {
            let sd = StatesAndDistricts::get().await?;
            println!("{}", sd.district_id(d.state.as_str(), d.district.as_str())?);
        }
    }
    Ok(())
}

#[derive(Clap, Debug)]
#[clap(name = "cowin-rs")]
struct App {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    DistrictId(DistrictId),
}

#[derive(Clap, Debug)]
struct DistrictId {
    #[clap(long, index(1))]
    state: String,
    #[clap(long, index(2))]
    district: String,
}
