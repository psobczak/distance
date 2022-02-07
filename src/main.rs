mod entities;

use entities::city::City;
use entities::units::DistanceUnit;
use entities::units::Latitude;
use structopt::StructOpt;

use crate::entities::units::Longitude;
use crate::entities::units::TryAdd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let opt = Opt::from_args();
    // println!("{:?}", opt);

    // let lat1 = Longitude(10.23);
    // let lat2 = Longitude(200.23);
    // println!("{:?}", lat1.try_add(lat2));

    // let wroclaw = City::new("Wrocław");
    // println!("{:?}", wroclaw);

    let lng = Longitude::try_from(190.0)?;
    println!("{:?}", lng);

    Ok(())
}

#[derive(Debug, StructOpt)]
struct Opt {
    first_city: String,
    second_city: String,
    #[structopt(short, long)]
    unit: Option<DistanceUnit>,
}
