
use cmake::Config;

fn main() {
    let destination = Config::new("BlingFire")
        .always_configure(true)
        .define("BLING_FIRE_VERSION_MAJOR", "1")
        .define("BLING_FIRE_VERSION_MINOR", "0")
        .build_target("blingfiretokdll_static")
        .build();