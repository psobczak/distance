mod entities;

use entities::city::City;
use entities::units::DistanceUnit;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wroclaw = City::new("Wrocław");
    println!("{}", wroclaw);

    Ok(())
}

#[derive(Debug, StructOpt)]
struct Opt {
    first_city: String,
    second_city: String,
    #[structopt(short, long)]
    unit: Option<DistanceUnit>,
}
