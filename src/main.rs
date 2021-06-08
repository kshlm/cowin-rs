use chrono::NaiveDate;
use clap::Clap;
use color_eyre::eyre::Result;

use cowin_rs::api::{appointment::Sessions, location::StatesAndDistricts};

#[async_std::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::init();

    let app = App::parse();
    match app.subcmd {
        SubCommand::DistrictId(d) => {
            let sd = StatesAndDistricts::get().await?;
            println!("{}", sd.district_id(d.state.as_str(), d.district.as_str())?);
        }
        SubCommand::Appointments(a) => {
            let sessions = if let Some(pincode) = a.pincode {
                Sessions::get_by_pin(&pincode, a.date).await?
            } else if let Some(district_id) = a.district_id {
                Sessions::get_by_district(district_id, a.date).await?
            } else {
                unreachable!();
            };
            dbg!(sessions);
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
    Appointments(Appointments),
}

#[derive(Clap, Debug)]
struct DistrictId {
    #[clap(long, short)]
    state: String,
    #[clap(long, short)]
    district: String,
}

#[derive(Clap, Debug)]
struct Appointments {
    #[clap(
        alias = "pin",
        long,
        required_unless_present = "district-id",
        conflicts_with = "district-id"
    )]
    pincode: Option<String>,
    #[clap(
        alias = "district",
        long,
        required_unless_present = "pincode",
        conflicts_with = "pincode"
    )]
    district_id: Option<i16>,
    #[clap(long)]
    date: Option<NaiveDate>,
}
