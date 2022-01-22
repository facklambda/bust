# bust

bust is a WORK IN PROGRESS bus timetable viewer for your terminal.

# Description

A [computer club](https://cyberia.club) I am part of has a [physical space](https://layerze.ro) that I would like to get to by bus, as there is a bus stop right outside. I figured it would be really cool to have a computer terminal at the space with a fully functional bus timetable installed. Since this doesn't exist yet I am making it!

Currently using this project to build my Rust muscles.

NOT YET compatible with the following public transit providers:
* Metro Transit


# misc notes

Structs I made for the response data structures, but might not be needed.

```rust
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
```


