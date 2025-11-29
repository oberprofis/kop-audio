fn main() {
    pkg_config::probe_library("libpulse-simple").unwrap();
}
