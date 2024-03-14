
use cmake::Config;

fn main() {
    let destination = Config::new("BlingFire")
        .always_configure(true)