use clap::{AppSettings, Parser};
use gtfs_structures::StopTime;
use tokio::select;

//const GTFS_ZIP: &str = "./gtfs/";
const GTFS_ZIP: &str = "https://svc.metrotransit.org/mtgtfs/gtfs.zip";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(setting(AppSettings::ArgRequiredElseHelp))]

struct Cli {
    /// Stop ID
    stop_id: String,

    /// Route name or number
    #[clap(short, long)]
    route_id: Option<String>,

    /// Direction of route
    #[clap(short, long, arg_enum)]
    direction: Option<Direction>,

    /// Force usage of Nextrip API instead of local GTFS archive
    #[clap(long)]
    nextrip: bool,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// Format output to JSON
    #[clap(long)]
    json: bool,
}

#[derive(clap::ArgEnum, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

//starting small, display bus info in terminal
fn main() {
    let cli = Cli::parse();

    if cli.nextrip {
        println!("local GTFS ignored, requesting data from NexTrip API");
        
    }

    if cli.route_id.is_some() && cli.direction.is_none() {
        println!(
            "Fetching timetable for route: {:?}, serving stop: {:?}",
            cli.route_id, cli.stop_id
        );
    } else if cli.direction.is_some() && cli.route_id.is_some() {
        println!(
            "Fetching timetable for {:?}bound route: {:?}, serving stop: {:?}",
            cli.direction, cli.route_id, cli.stop_id
        );
    } else if cli.direction.is_some() && cli.route_id.is_none() {
        println!(
            "Fetching timetable for all {:?}bound routes serving stop: {:?}",
            cli.direction, cli.stop_id
        );
    } else {
        println!("Fetching timetable for the stop: {:?}", cli.stop_id);
    }
    // commenting this out because it makes the test runs take like 30 seconds (fetching the gtfs zip, unzipping, and then parsing seems to be quite slow)
    // let gtfs = gtfs_structures::GtfsReader::default()
    //     .read_stop_times(false)
    //     .read(GTFS_ZIP)
    //     .expect("impossible to read gtfs");
    // gtfs.print_stats();

    // println!("stop data: {:?}", gtfs.get_stop(&cli.stop_id));
}
