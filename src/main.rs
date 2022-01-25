use clap::{AppSettings, Parser};
use gtfs_structures::StopTime;
use tokio::select;

const GTFS_ZIP: &str = "./gtfs/"; //"https://svc.metrotransit.org/mtgtfs/gtfs.zip";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(setting(AppSettings::ArgRequiredElseHelp))]

struct Cli {
    ///Stop ID
    #[clap(long)]
    stop_id: Option<String>,

    /// Route name or number
    #[clap(long)]
    route_id: Option<String>,

    /// Direction of route
    #[clap(long)]
    direction: Option<String>,

    /// Toggle pretty interface
    #[clap(long)]
    pretty: bool,

    /// Toggle audible and visible alerts
    #[clap(long)]
    alert: bool,

    /// Toggle verbosity
    #[clap(long)]
    verbose: bool,

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
    let gtfs = gtfs_structures::GtfsReader::default()
        .read_stop_times(false)
        .read(GTFS_ZIP)
        .expect("impossible to read gtfs");
    gtfs.print_stats();

    if let Some(i) = cli.stop_id {
        println!("stop id : {:?}", i.parse::<i64>().unwrap());
        println!("stop data: {:?}", gtfs.get_stop(&i));
    }

}
