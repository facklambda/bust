use clap::{AppSettings, Parser};
use tokio::select;
use gtfs_structures::Gtfs;

const GTFS_ZIP: &str = "https://svc.metrotransit.org/mtgtfs/gtfs.zip";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(setting(AppSettings::ArgRequiredElseHelp))]

struct Cli {
    ///stop id
    #[clap(long)]
    stop_id: Option<u64>,

    /// route name or number
    #[clap(long)]
    route: Option<String>,

    /// direction of route
    #[clap(long)]
    direction: Option<String>,

    /// toggle pretty interface
    #[clap(long)]
    pretty: bool,

    /// toggle audible and visible alerts
    #[clap(long)]
    alert: bool,

    /// toggle verbosity
    #[clap(long)]
    verbose: bool,

    /// set output format
    #[clap(short, long)]
    output: Option<String>,

    /// set output limit
    #[clap(short, long)]
    limit: Option<u64>,
}

//starting small, display bus info in terminal
fn main() {
    let cli = Cli::parse();
    
    if cli.verbose == true {
        println!("Printing stats:");
        let gtfs = gtfs_structures::Gtfs::new(GTFS_ZIP).expect("error reading zip from url");
        gtfs.print_stats();
        println!("verbosity is enabled, you will now see all println! debugging");
        if cli.alert == true {
            println!("Alerting is enabled, maybe a beep will happen or something");
        } else {
            println!("alerting is not enabled");
        }
    } else {
        println!("eventually this will print useful data!")
    }
}
