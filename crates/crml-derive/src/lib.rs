use serde::{Serialize, Deserialize};
use pathbufd::PathBufD;

use std::sync::LazyLock;
use std::fs::{read_to_string, File};

mod generator;

/// The `./crml.json` file.
#[derive(Serialize, Deserialize, Clone)]
struct Config {
    /// The root directory to load all templates from.
    ///
    /// # Example
    /// ```json
    /// {
    ///     "root_dir": "./templates"
    /// }
    /// ```
    pub root_dir: PathBufD,
}

// read config to constant
static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    serde_json::from_str::<Config>(
        &read_to_string(PathBufD::current().join("crml.json"))
            .expect("failed to read configuration"),
    )
    .expect("failed to deserialize configuration")
});

// macro
use syn::{parse_macro_input, LitStr, ItemStruct};
use quote::{quote, ToTokens};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

fn get_file(file: String) -> File {
    File::open(PathBufD::current().extend(&[CONFIG.root_dir.to_string(), format!("{}.crml", file)]))
        .expect("failed to read included file")
}

#[proc_macro_attribute]
pub fn template(args: TokenStream, input: TokenStream) -> TokenStream {
    // parse args
    let args = parse_macro_input!(args as LitStr);
    let file_name = args.value();

    // parse tokens
    let input = parse_macro_input!(input as ItemStruct);

    let struct_ident = input.ident.clone();
    let mut struct_tokens = TokenStream2::new();
    input.to_tokens(&mut struct_tokens);

    // read file into generator
    let generated: TokenStream2 = generator::Generator::from_file(get_file(file_name))
        .consume()
        .parse()
        .unwrap();

    // build output
    let expanded = quote! {
        #struct_tokens

        impl crml::Template for #struct_ident {
            fn render(self) -> String {
                #generated
            }
        }
    };

    // return
    TokenStream::from(expanded)
}
