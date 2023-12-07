use vergen::{vergen, Config, TimeZone, ShaKind};

fn main() {
    // Generate the default 'cargo:' instruction output
    let mut config = Config::default();
    *config.build_mut().timezone_mut() = TimeZone::Local;
    *config.git_mut().sha_kind_mut() = ShaKind::Short;
    vergen(config).ok();
}
