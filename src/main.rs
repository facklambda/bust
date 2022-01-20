use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

const PROVIDERS_DB: &str = "./bust/providers.json"; //updates daily
const ROUTES_DB: &str = "./bust/routes.json"; //updates daily
const DIRECTIONS_DB: &str = "./bust/directions.json"; //updates daily
const STOPS_DB: &str = "./bust/stops.json"; //updates daily lazily
const DEPARTURES_DB: &str = "./bust/departures.json"; //updates every 30s while viewed
const TIMEPOINT_DB: &str = "./bust/timepoint.json" //updates every 30s while viewed
const LOCATIONS_DB: &str = "./bust/locations.json" //updates every 30s while viewed

#[derive(Serialize, Deserialize, Clone)]
struct Route {
    description: String,
    provider_id: String,
    route: String,
}

struct Departure {
    actual: bool;
    block_number: u64;
    departure_text: String;
    departure_time: String;
    description: String;
    gate: String;
    route: String;
    route_direction: String;
    terminal: String;
    vehicle_heading: u64;
    vehicle_latitude: f64;
    vehicle_longitude: f64;
}

struct Pair {
    text: String;
    value: String;
}

struct Locations {
    bearing: f64;
	block_number: u64;
	direction: u64;
	location_time: String;
	odometer: u64;
	route: String;
	speed: f64;
	terminal:String;
	vehicle_latitude: f64;
	vehicle_longitude: f64;

}


struct Route
fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;
    Ok(())
}

fn {}