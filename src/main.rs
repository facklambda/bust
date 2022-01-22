use clap::{Parser, Args, Subcommand, ArgEnum};


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Name or number of the route to query
    #[clap(short, long)]
    route: Option<String>,

    /// Stop ID to query
    #[clap(short, long)]
    stop_id: Option<u64>,

    // One of either of the two directions that are valid for a given route. Either North/South or East/West. Used to filter output from other flags.
    #[clap(short, long)]
    direction: Option<String>,
}


//starting small, display bus info in terminal
fn main() {
    let cli = Cli::parse();

    println!("eventually this will print useful data!")
}

