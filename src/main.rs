use muzzle::{generate_usage, get_opts, run, Units};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = get_opts();

    // Get matches
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("Failed to parse parameters with: {}", f.to_string());
            process::exit(1);
        },
    };

    // Display help and exit
    if matches.opt_present("h") {
        println!("{}", generate_usage(&opts));
        process::exit(0);
    }

    let result = match run(matches) {
        Ok(params) => params,
        Err(error) => {
            println!("Failed to calculate parameters with: {}", error);
            process::exit(1);
        },
    };

    let mass_unit: &str;
    let speed_unit: &str;
    let energy_unit: &str;

    // Figure out units
    match result.units {
        Units::METRIC => {
            mass_unit = "grams";
            speed_unit = "m/s";
            energy_unit = "Joules";
        },
        Units::IMPERIAL => {
            mass_unit = "grains";
            speed_unit = "FPS";
            energy_unit = "FPE";
        },
    }

    print!(
        "Derived shot parameters are:\nProjectile mass:\t{:.3} {}\nProjectile speed:\t{:.3} {}\nProjectile energy:\t{:.3} {}\n\n",
        result.mass, mass_unit,
        result.speed, speed_unit,
        result.energy, energy_unit
    );

    process::exit(0);
}
