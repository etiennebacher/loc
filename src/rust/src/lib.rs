use extendr_api::prelude::*;
use tokei::{Config, LanguageType, Languages};

#[extendr]
fn count_loc() -> f64 {
    // The paths to search. Accepts absolute, relative, and glob paths.
    let paths = &["src", "tests"];
    // Exclude any path that contains any of these strings.
    let excluded = &["target"];
    // `Config` allows you to configure what is searched and counted.
    let config = Config::default();

    let mut languages = Languages::new();
    languages.get_statistics(paths, excluded, &config);
    let rust = &languages[&LanguageType::Rust];
    rust.code as f64
}

extendr_module! {
    mod loc;
    fn count_loc;
}
