use getopts::{Options, Matches};

const GEE_FPS: f64 = 32.174;
const GRAMS_IN_KILO: f64 = 1000f64;
const GRANS_IN_POUND: f64 = 7000f64;

pub enum Units {
    METRIC,
    IMPERIAL,
}

pub struct Params {
    pub units: Units,
    pub mass: f64,
    pub speed: f64,
    pub energy: f64,
}

// Performs calculations
pub fn run(matches: Matches) -> Result<Params, String> {
    let units = if matches.opt_present("i") { Units::IMPERIAL } else { Units::METRIC };
    
    let mass = get_float(matches.opt_str("m"))?;
    let speed = get_float(matches.opt_str("s"))?;
    let energy = get_float(matches.opt_str("e"))?;

    if mass.is_some() && speed.is_some() {
        let derived_energy = derive_energy(mass.unwrap(), speed.unwrap(), &units);

        return Ok(Params {
            units,
            mass: mass.unwrap(),
            speed: speed.unwrap(),
            energy: derived_energy,
        });
    }
    else if mass.is_some() && energy.is_some() {
        let derived_speed = derive_speed(mass.unwrap(), energy.unwrap(), &units);

        return Ok(Params {
            units,
            mass: mass.unwrap(),
            speed: derived_speed,
            energy: energy.unwrap(),
        });
    }
    else if speed.is_some() && energy.is_some() {
        let derived_mass = derive_mass(speed.unwrap(), energy.unwrap(), &units);

        return Ok(Params {
            units,
            mass: derived_mass,
            speed: speed.unwrap(),
            energy: energy.unwrap(),
        });
    }

    Err(
        "Incorrect parameters set. At least two out of three parameters must be given to derive the third.
Please check input.".to_owned()
    )
}

/// Defines acceptable flags and options and returns complete `Options` object
pub fn get_opts() -> Options {
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("i", "imperial", "use imperial units instead of metric");

    opts.optopt("m", "mass", "mass of the projectile (grains for imperial or grams for metric)", "NUMBER");
    opts.optopt("s", "speed", "velocity of the projectile (FPS for imperial or m/s for metric)", "NUMBER");
    opts.optopt("e", "energy", "muzzle energy of the projectile (FPE for imperial or Joules for metric)", "NUMBER");

    opts
}

/// Generates usage information string out of options object
pub fn generate_usage(opts: &Options) -> String {
    let brief = "USAGE: muzzle [--imperial] [--mass NUMBER] [--speed NUMBER] [--energy NUMBER]
\nEnter either two of the three parameters to get the third.";
    opts.usage(&brief)
}

/// Derives mass from given `speed` and `energy` using set units of measurment
fn derive_mass(speed: f64, energy: f64, units: &Units) -> f64 {
    match units {
        Units::METRIC => ((2.0 * energy) / speed.powi(2)) * GRAMS_IN_KILO,
        Units::IMPERIAL => ((2.0 * GEE_FPS * energy) / speed.powi(2)) * GRANS_IN_POUND,
    }
}

/// Derives mass from given `speed` and `energy` using set units of measurment
fn derive_speed(mass: f64, energy: f64, units: &Units) -> f64 {
    let speed_squared = match units {
        Units::METRIC => (2.0 * energy) / (mass / GRAMS_IN_KILO),
        Units::IMPERIAL => (2.0 * GEE_FPS *energy) / (mass / GRANS_IN_POUND),
    };

    speed_squared.powf(0.5)
}

/// Derives mass from given `speed` and `energy` using set units of measurment
fn derive_energy(mass: f64, speed: f64, units: &Units) -> f64 {
    match units {
        Units::METRIC => ((mass / GRAMS_IN_KILO) * speed.powi(2)) / 2.0,
        Units::IMPERIAL => ((mass / GRANS_IN_POUND) * speed.powi(2)) / (2.0 * GEE_FPS),
    }
}

/// Tries to parse `f64` out of given option string
fn get_float(param: Option<String>) -> Result<Option<f64>, String> {
    match param {
        Some(st) => match st.parse() {
            Ok(n) => Ok(Some(n)),
            Err(_) => Err(format!("Failed to parse `{}` as input parameter!", st)),
        },
        None => Ok(None),
    }
}
