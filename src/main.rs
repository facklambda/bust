use clap::{AppSettings, Parser};
use gtfs_structures::StopTime;
use tokio::select;

const GTFS_ZIP: &str = "./gtfs/"; //"https://svc.metrotransit.org/mtgtfs/gtfs.zip";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(setting(AppSettings::ArgRequiredElseHelp))]

struct Cli {
    /// Stop ID
    stop_id: String,

    /// Route name or number
    #[clap(long)]
    route_id: Option<String>,

    /// Direction of route
    #[clap(long)]
    direction: Option<String>,

    /// Force usage of Nextrip API instead of local GTFS archive
    #[clap(long)]
    nextrip: bool,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// Set output format
    #[clap(short, long)]
    output: Option<String>,

    /// Set output limit
    #[clap(short, long)]
    limit: Option<u64>,
}

//starting small, display bus info in terminal
fn main() {
    let cli = Cli::parse();
    // let gtfs = gtfs_structures::GtfsReader::default()
    //     .read_stop_times(false)
    //     .read(GTFS_ZIP)
    //     .expect("impossible to read gtfs");
    // gtfs.print_stats();

    println!("Fetching timetable for the Stop ID: {:?}", cli.stop_id);
    println!("Fetching timetable for Route: {:?}, serving Stop ID: {:?}", cli.route_id, cli.stop_id);
    println!("Fetching timetable for {:?}bound Route: {:?}, serving Stop ID: {:?}", cli.direction, cli.route_id, cli.stop_id);
    println!("Fetching timetable for all {:?}bound routes serving Stop ID: {:?}", cli.direction, cli.stop_id);
}
