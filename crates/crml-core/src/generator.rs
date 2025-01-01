use crate::{TokenStream, TokenType, Parser};
use std::{fs::File, io::Read};

/// Generate valid Rust from a given [`TokenStream`].
pub struct Generator(TokenStream);

impl Generator {
    /// Create a new [`Generator`].
    pub fn new(input: TokenStream) -> Self {
        Self(input)
    }

    /// Create a new [`Generator`] from a [`File`].
    pub fn from_file(mut file: File) -> Self {
        // read file
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("failed to read file");

        // return
        Self(Parser::new(content).parse())
    }

    /// Generate valid Rust from the given `input`.
    ///
    /// The function output has a single argument with the type of the given `props_type`.
    /// This type is imported from `crate::crml::data::*`. That module should export all
    /// types which are going to be used as page data.
    ///
    /// The example below shows you how to use the data given from calling `consume` with
    /// the name of `test_template` and the `props_type` of `TestProps`.
    ///
    /// After building all templates, the `src/crml/mod.rs` file should be created
    /// with the combined contents of all functions, as well as the `use super::data::*` line
    /// in order to satisfy all type requirements.
    ///
    /// ```rust
    /// //! main.rs
    /// mod crml;
    ///
    /// pub(crate) struct TestProps {
    ///     a: i32
    /// }
    ///
    /// fn main() {
    ///     println!("rendered: {}", crml::test_template(TestProps {
    ///         a: 1
    ///     }))
    /// }
    /// ```
    ///
    /// ```rust
    /// //! crml/mod.rs
    /// // @generated crml build
    /// use super::data::*;
    /// pub fn test_template(page: TestProps) -> String {
    ///     // ...
    /// }
    /// ```
    ///
    /// ```rust
    /// //! crml/data.rs - this should be written before building crml templates
    /// pub use crate::TestProps;
    /// ```
    pub fn consume(mut self, name: String, props_type: String) -> String {
        let mut out = format!(
            "/// Render the `{name}.crml` template with the given [`{props_type}`] properties.
///
/// # Arguments
/// * `page` - [`{props_type}`]
///
/// # Returns
/// Rendered string.
///
/// # Example
/// ```rust
/// println!(\"rendered: {{}}\", {name}({props_type}::default()));
/// ```
pub fn {name}(page: {props_type}) -> String {{
    let mut crml_rendered = String::new();\n"
        )
        .to_string();

        while let Some(mut token) = self.0.next() {
            match token.r#type {
                TokenType::RustString => {
                    if !token.raw.ends_with("{") && token.raw != "}" {
                        token.raw += ";";
                    }

                    out.push_str(&(token.raw + "\n"));
                }
                _ => {
                    if token.raw == "\n" {
                        continue;
                    }

                    out.push_str(&format!(
                        "crml_rendered.push_str(&format!(\"{}\"));\n",
                        token.html.replace('"', "\\\"")
                    ));
                }
            }
        }

        format!("{out}crml_rendered\n}}")
    }
}
