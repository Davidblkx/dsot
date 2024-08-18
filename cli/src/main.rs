use dsot_core::setup::{SetupConfig, LibConfig};

fn main() {
    dsot_core::setup(SetupConfig {
        local_data: std::path::PathBuf::from("./"),
        lib: LibConfig {
            name: "library".to_string(),
        },
    }).expect("Failed to setup DSOT");
}
