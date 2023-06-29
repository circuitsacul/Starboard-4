fn main() {
    println!("cargo:rerun-if-changed=migrations");

    rosetta_build::config()
        .source("en", "locales/en.json")
        .source("pt", "locales/pt.json")
        .fallback("en")
        .generate()
        .unwrap();
}
