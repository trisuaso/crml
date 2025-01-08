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

pub(crate) fn get_file(file: &str) -> File {
    File::open(PathBufD::current().extend(&[CONFIG.root_dir.to_string(), format!("{}.crml", file)]))
        .expect("failed to read included file")
}

// yes this is an attribute macro and not a derive macro, it used to be derive
/// Mark a struct as a template and provide the name of the template file it uses.
///
/// # Example
/// ```rust
/// use crml::{template, Template}; // import template macro *and* Template trait
///
/// #[template("mycrmlfile")]
/// struct MyStruct {
///     a: i32
/// }
///
/// fn main() {
///     // the Template trait provides .render()
///     println!("rendered: {}", MyStruct { a: 1 }.render());
/// }
/// ```
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
    let generated = generator::Generator::from_file(get_file(&file_name)).consume();

    let generated_tokens: TokenStream2 = match generated.parse() {
        Ok(t) => t,
        Err(e) => {
            // debug outputs
            if std::fs::exists("crml_dbg").expect("failed to check for debug dir") == true {
                println!(
                    "Debug directory found. Check \"crml_dbg/{}.rs\" to debug template.",
                    file_name
                );

                std::fs::write(
                    format!("crml_dbg/{file_name}.rs"),
                    format!("fn debug() {{\n{generated}\n}}"),
                )
                .expect("failed to write debug file")
            }

            // panic :(
            panic!("{}", e.to_string())
        }
    };

    // build output
    let expanded = quote! {
        #struct_tokens

        impl crml::Template for #struct_ident {
            fn render(self) -> String {
                #generated_tokens
            }
        }
    };

    // debug outputs
    if std::fs::exists("crml_dbg").expect("failed to check for debug dir") == true {
        let file_name = file_name.replace("/", "_");
        std::fs::write(format!("crml_dbg/{file_name}.rs"), expanded.to_string())
            .expect("failed to write debug file")
    }

    // return
    expanded.into()
}
