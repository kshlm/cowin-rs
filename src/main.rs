use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};
use cli_table::{print_stdout, WithTitle};
use eyre::Result;

use cowin_rs::api::{appointment::Sessions, location::StatesAndDistricts};

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

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
            if !sessions.is_empty() {
                assert!(print_stdout(sessions.with_title()).is_ok());
            } else {
                println!("No appointments found");
            }
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "cowin-rs")]
struct App {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    DistrictId(DistrictId),
    Appointments(Appointments),
}

#[derive(Args, Debug)]
struct DistrictId {
    #[arg(long, short)]
    state: String,
    #[arg(long, short)]
    district: String,
}

#[derive(Args, Debug)]
struct Appointments {
    #[arg(
        alias = "pin",
        long,
        required_unless_present = "district_id",
        conflicts_with = "district_id"
    )]
    pincode: Option<String>,
    #[arg(
        alias = "district",
        long,
        required_unless_present = "pincode",
        conflicts_with = "pincode"
    )]
    district_id: Option<i16>,
    #[arg(long)]
    date: Option<NaiveDate>,
}