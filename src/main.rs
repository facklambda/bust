use clap::{Parser};
use dialoguer::{Confirm, FuzzySelect, Select};
use tokio::select;


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {

    ///Stop ID
    #[clap(short, long)]
    stop: Option<u64>,

    /// Route name or number
    #[clap(short, long)]
    route: Option<String>,

    /// Direction of route
    #[clap(short, long)]
    direction: Option<String>,

    /// Toggles pretty interface
    #[clap(short, long)]
    pretty: bool,

    /// Toggles audible and visible alert
    #[clap(short, long)]
    alert: bool,
    
    /// Set output format
    #[clap(short, long)]
    output: Option<String>,

    /// Limit output length
    #[clap(short, long)]
    limit: Option<u64>

}


//starting small, display bus info in terminal
fn main() {
    let cli = Cli::parse();

    println!("eventually this will print useful data!")
}

