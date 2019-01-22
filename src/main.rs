use getopts::Options;
use muzzle::{run, Config, Units};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("i", "imperial", "use imperial units instead of metric");

    opts.optopt("m", "mass", "mass of the projectile (grains for imperial or grams for metric)", "NUMBER");
    opts.optopt("s", "speed", "velocity of the projectile (FPS for imperial or m/s for metric)", "NUMBER");
    opts.optopt("e", "energy", "muzzle energy of the projectile (FPE for imperial or Joules for metric)", "NUMBER");

    // Get matches
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("Failed to parse parameters with: {}", f.to_string());
            process::exit(1);
        },
    };

    // Display help and exit
    if matches.opt_present("h") {
        println!("{}", generate_usage(&opts));
        process::exit(0);
    }

    let config = Config {
        units: if matches.opt_present("i") { Units::IMPERIAL } else { Units::METRIC },
        mass: matches.opt_str("m"),
        speed: matches.opt_str("s"),
        energy: matches.opt_str("e"),
    };

    let result = match run(config) {
        Ok(params) => params,
        Err(error) => {
            eprintln!("Failed to calculate parameters with: {}", error);
            process::exit(1);
        },
    };

    // Figure out units
    let (mass_unit, speed_unit, energy_unit) = match result.units {
        Units::METRIC => ("grams", "m/s", "Joules"),
        Units::IMPERIAL => ("grains", "FPS", "FPE"),
    };

    if result.bogus {
        println!("WARNING: All shot parameters have been given. Nothing has been derived. Displaying as is.");
    }

    print!(
        "Derived shot parameters are:\nProjectile mass:\t{:.3} {}\nProjectile speed:\t{:.3} {}\nProjectile energy:\t{:.3} {}\n\n",
        result.mass, mass_unit,
        result.speed, speed_unit,
        result.energy, energy_unit
    );

    process::exit(0);
}

/// Generates usage information string out of options object
fn generate_usage(opts: &Options) -> String {
    let brief = "USAGE: muzzle [--imperial] [--mass NUMBER] [--speed NUMBER] [--energy NUMBER]
\nEnter either two of the three parameters to get the third.";
    opts.usage(&brief)
}
