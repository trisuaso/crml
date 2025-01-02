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

        let mut last_indent_level: usize = 0;
        let mut last_tag: String = String::new();
        let whitespace_sensitive = &["script", "style"]; // these must be closed manually

        while let Some(mut token) = self.0.next() {
            if (token.indent < last_indent_level)
                && !last_tag.is_empty()
                && !whitespace_sensitive.contains(&last_tag.as_str())
            {
                // automatically close previous element
                out.push_str(&format!(
                    "crml_rendered.push_str(&format!(\"</{last_tag}>\"));\n"
                ));

                last_tag = String::new();
            }

            last_indent_level = token.indent;
            match token.r#type {
                TokenType::RustString => {
                    if !token.raw.ends_with("{") && token.raw != "}" {
                        token.raw += ";";
                    }

                    out.push_str(&format!("{}//line: {}\n", token.raw, token.line));
                }
                TokenType::PushedRustString => {
                    out.push_str(&format!(
                        "crml_rendered.push_str(&{});//line: {}\n",
                        token.raw, token.line
                    ));
                }
                _ => {
                    if token.raw == "\n" {
                        continue;
                    } else if token.raw == "end" {
                        out.push_str(&format!(
                            "crml_rendered.push_str(&format!(\"</{last_tag}>\"));\n"
                        ));

                        continue;
                    }

                    if let Some(selector) = token.selector {
                        last_tag = selector.tag;
                    }

                    if token.html.contains("</") {
                        // token closed tag itself
                        last_tag = String::new();
                    }

                    if whitespace_sensitive.contains(&last_tag.as_str()) {
                        // whitespace sensitive blocks do not accept format params
                        token.html = token.html.replace("{", "{{").replace("}", "}}")
                    }

                    out.push_str(&format!(
                        "crml_rendered.push_str(&format!(\"{}\"));//line: {}\n",
                        token.html.replace('"', "\\\""),
                        token.line
                    ));
                }
            }
        }

        format!("{out}crml_rendered\n}}\n")
    }
}
