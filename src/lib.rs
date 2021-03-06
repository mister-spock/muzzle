const GEE_FPS: f64 = 32.174;
const GRAMS_IN_KILO: f64 = 1000f64;
const GRAINS_IN_POUND: f64 = 7000f64;

/// Measurment system to perform calculations in.
pub enum Units {
    /// Metric system (e.g. "meters per secons", "joules", "grams")
    METRIC,
    /// Imperial system (e.g. "feet per second", "foot-pounds of energy", "grains")
    IMPERIAL,
}

/// Configuration object that needs to be passed to `run` function to perform calculations on.
pub struct Config {
    /// Chosen units of measurment
    pub units: Units,
    /// Mass of the projectile (in grams of grains depending on measurment system chosen)
    pub mass: Option<String>,
    /// Speed of the projectile (in m/s of FPS depending on measurment system chosen)
    pub speed: Option<String>,
    /// Energy of the projectile (in joules of FPE depending on measurment system chosen)
    pub energy: Option<String>,
}

/// Object that is the output of the `run` function.
/// Holds all derived parameters of the shot.
pub struct Params {
    /// Chosen units of measurment
    pub units: Units,
    /// Mass of the projectile (in grams of grains depending on measurment system chosen)
    pub mass: f64,
    /// Speed of the projectile (in m/s of FPS depending on measurment system chosen)
    pub speed: f64,
    /// Energy of the projectile (in joules of FPE depending on measurment system chosen)
    pub energy: f64,
    /// A flag that points to the fact that `run` function got all three of the input parameters
    /// as input in `Config` object and no calculations have been performed
    pub bogus: bool,
}

/// Performs calculations based on given input config.
/// Returns either shot parameters struct, or an error string.
pub fn run(config: Config) -> Result<Params, String> {
    let units = config.units;

    match (get_float(config.mass)?, get_float(config.speed)?, get_float(config.energy)?) {
        // Mass and speed given. Derive energy.
        (Some(m), Some(s), None) => {
            let derived_energy = derive_energy(&m, &s, &units);
            Ok(Params { units, mass: m, speed: s, energy: derived_energy, bogus: false })
        },
        // Mass and energy given. Derive speed.
        (Some(m), None, Some(e)) => {
            let derived_speed = derive_speed(&m, &e, &units);
            Ok(Params { units, mass: m, speed: derived_speed, energy: e, bogus: false })
        },
        // Speed and energy given. Derive mass.
        (None, Some(s), Some(e)) => {
            let derived_mass = derive_mass(&s, &e, &units);
            Ok(Params { units, mass: derived_mass, speed: s, energy: e, bogus: false })
        },
        // All parameters passed. Nothing to derive.
        (Some(m), Some(s), Some(e)) => Ok(Params { units, mass: m, speed: s, energy: e, bogus: true }),
        // Everything else is an error.
        _ => Err(
            "Incorrect parameters set. At least two out of three parameters must be given to derive the third.
Please check input.".to_owned()
        ),
    }
}

/// Derives mass from given `speed` and `energy` using set units of measurment
fn derive_mass(speed: &f64, energy: &f64, units: &Units) -> f64 {
    match units {
        Units::METRIC => ((2.0 * energy) / speed.powi(2)) * GRAMS_IN_KILO,
        Units::IMPERIAL => ((2.0 * GEE_FPS * energy) / speed.powi(2)) * GRAINS_IN_POUND,
    }
}

/// Derives mass from given `speed` and `energy` using set units of measurment
fn derive_speed(mass: &f64, energy: &f64, units: &Units) -> f64 {
    let speed_squared = match units {
        Units::METRIC => (2.0 * energy) / (mass / GRAMS_IN_KILO),
        Units::IMPERIAL => (2.0 * GEE_FPS * energy) / (mass / GRAINS_IN_POUND),
    };

    speed_squared.powf(0.5)
}

/// Derives mass from given `speed` and `energy` using set units of measurment
fn derive_energy(mass: &f64, speed: &f64, units: &Units) -> f64 {
    match units {
        Units::METRIC => ((mass / GRAMS_IN_KILO) * speed.powi(2)) / 2.0,
        Units::IMPERIAL => ((mass / GRAINS_IN_POUND) * speed.powi(2)) / (2.0 * GEE_FPS),
    }
}

/// Tries to parse `f64` out of given option string
fn get_float(param: Option<String>) -> Result<Option<f64>, String> {
    match param {
        Some(st) => match st.parse::<f64>() {
            Ok(n) => Ok(Some(n)),
            Err(_) => Err(format!("Failed to parse `{}` as input parameter!", st)),
        },
        None => Ok(None),
    }
}
