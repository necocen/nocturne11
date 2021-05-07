use vergen::{vergen, Config, TimeZone};

fn main() {
    // Generate the default 'cargo:' instruction output
    let mut config = Config::default();
    *config.build_mut().timezone_mut() = TimeZone::Local;
    vergen(config).ok();
}
