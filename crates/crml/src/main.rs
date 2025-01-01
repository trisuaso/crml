use crml_core;
use std::fs::{read_to_string, write, File};

use serde::{Serialize, Deserialize};
use pathbufd::PathBufD;

/// The `./crml.json` file.
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    /// The directory to generate the `crml` module in.
    ///
    /// # Example
    /// ```json
    /// {
    ///     "output_dir": "./src"
    /// }
    /// ```
    pub output_dir: PathBufD,
    /// The root directory to load all templates from.
    ///
    /// # Example
    /// ```json
    /// {
    ///     "root_dir": "./templates"
    /// }
    /// ```
    pub root_dir: PathBufD,
    /// The templates to include (with their props type name).
    ///
    /// # Example
    /// ```json
    /// {
    ///     "include": [
    ///         ["index", "IndexProps"]
    ///     ]
    /// }
    /// ```
    ///
    /// All file names are assumed to also include the `.crml` extension.
    pub include: Vec<(String, String)>,
}

fn main() {
    // read configuration
    let config: Config = serde_json::from_str(
        &read_to_string(PathBufD::current().join("crml.json"))
            .expect("failed to read configuration"),
    )
    .expect("failed to deserialize configuration");

    // compile
    let mut out = format!("// This file is @generated.\nmod data;\nuse data::*;\n");

    for file in config.include {
        let fs_file = File::open(PathBufD::current().extend(&[
            config.root_dir.to_string(),
            format!("{}.crml", file.0.clone()),
        ]))
        .expect("failed to read included file");

        out.push_str(&crml_core::generator::Generator::from_file(fs_file).consume(file.0, file.1));
    }

    // write file
    write(
        PathBufD::current().extend(&[config.output_dir.to_string(), "mod.rs".to_string()]),
        out,
    )
    .expect("failed to write output file")
}
