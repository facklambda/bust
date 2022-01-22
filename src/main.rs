use clap::{Parser, Args, AppSettings, Subcommand, ArgEnum};


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    /// Provides info about a specified route.
    Route {
        /// Name or number of the route to query
        route: Option<String>,
    },

    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    /// Provides info about a specified stop
    Stop {
       /// 5 digit Stop ID to query
       stop_id: Option<u64>,
    },

    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    /// Provides info regarding your next specified trip
    Next {
        /// Name or number of the route to query
        route: Option<String>,
        /// Direction of route to query for
        direction: Option<String>,
        /// 5 digit Stop ID to query
        stop_id: Option<u64>,
    },
}


//starting small, display bus info in terminal
fn main() {
    let cli = Cli::parse();

    println!("eventually this will print useful data!")
}

