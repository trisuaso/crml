use crml_core::{TokenStream, TokenType, Parser};
use std::{fs::File, io::Read};

static RAW_BLOCK_TAG_PREFIX: &str = "r:";
static SLOT_BLOCK_TAG_PREFIX: &str = "s:";

/// Generate valid Rust from a given [`TokenStream`].
pub struct Generator(TokenStream);

impl Generator {
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
    pub fn consume(mut self) -> String {
        let mut out = format!("let mut crml_rendered = String::new();\nlet mut crml_templ_stack: Vec<String> = Vec::new();\n").to_string();

        let mut last_indent_levels: Vec<usize> = Vec::new();
        let mut last_tags: Vec<String> = Vec::new();
        let whitespace_sensitive = &["script", "style", "pre", "html", "body", "head"]; // these must be closed manually

        while let Some(mut token) = self.0.next() {
            let mut last_tag = last_tags.last().unwrap_or(&String::new()).to_owned();
            let last_indent_level = last_indent_levels.last().unwrap_or(&0).to_owned();

            if (token.indent < last_indent_level)
                && !last_tag.is_empty()
                && !whitespace_sensitive.contains(&last_tag.as_str())
                && !last_tag.starts_with(RAW_BLOCK_TAG_PREFIX)
                && !last_tag.starts_with(SLOT_BLOCK_TAG_PREFIX)
            {
                // automatically close previous element
                out.push_str(&format!(
                    "crml_rendered.push_str(&format!(\"</{last_tag}>\"));\n"
                ));

                last_tags.pop();
                last_indent_levels.pop();
            }

            if token.indent != last_indent_level {
                // push this indent level to the stack
                last_indent_levels.push(token.indent);
            }

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
                        out.push_str(&format!("crml_rendered.push_str(\"\\n\");\n"));
                        continue;
                    }

                    if let Some(selector) = token.selector {
                        if !selector.tag.starts_with("-") {
                            last_tags.push(selector.tag.clone());
                            last_tag = selector.tag.clone();

                            if selector.tag == "slot" {
                                // don't render <slot /> elements,
                                // they should be literally insertted into the rust
                                // in order to be replaced later
                                out.push_str(&format!(
                                    "crml_rendered.push_str(\"<slot {}/>\");\n",
                                    selector
                                        .attributes
                                        .unwrap()
                                        .get(0)
                                        .unwrap()
                                        .replace("\"", "\\\"")
                                ));
                                last_tags.pop();
                                continue;
                            }

                            if selector.tag.starts_with(RAW_BLOCK_TAG_PREFIX) {
                                // don't actually render this!
                                continue;
                            }

                            if selector.tag.starts_with(SLOT_BLOCK_TAG_PREFIX) {
                                // this is a slot for accepting another template as a base
                                let classes = selector.classes.unwrap();

                                let file_name = selector.tag.replace("s:", "");
                                let name = classes.get(0).unwrap();

                                // read file
                                let generated =
                                    Generator::from_file(crate::get_file(&file_name)).consume();

                                // push block
                                // in this block, we use the generated template and then rebuild
                                // crml_rendered with both parts of the template surrounding the current
                                // content that we have rendered
                                out.push_str(&format!(
                                    "let template_ = {{\n{generated}\n}};
                                    let template_split_: Vec<&str> = template_.split(\"<slot name=\\\"{name}\\\"/>\").collect();
                                    let template_half_0_ = template_split_.get(0).unwrap();
                                    let template_half_1_ = template_split_.get(1).unwrap();
                                    crml_rendered = format!(\"{{template_half_0_}}\n{{crml_rendered}}\n\");\n
                                    crml_templ_stack.push(template_half_1_.to_string());\n"
                                ));
                                // we push the SECOND HALF of the template to crml_templ_stack
                                // because that vector is all added (in order of push) to
                                // the output string
                                //
                                // this is done so that everything added after is still
                                // rendered into the correct template

                                // continue
                                continue;
                            }
                        } else {
                            last_tags.pop();

                            if selector
                                .tag
                                .starts_with(&format!("-{RAW_BLOCK_TAG_PREFIX}"))
                            {
                                // don't actually render this!
                                continue;
                            }
                        }
                    }

                    if token.html.contains("</") && !last_tag.starts_with(RAW_BLOCK_TAG_PREFIX) {
                        // token closed tag itself
                        last_tags.pop();
                    }

                    if whitespace_sensitive.contains(&last_tag.as_str())
                        | last_tag.starts_with(RAW_BLOCK_TAG_PREFIX)
                    {
                        // whitespace sensitive blocks do not accept format params
                        token.html = token.html.replace("{", "{{").replace("}", "}}")
                    }

                    if token.raw.starts_with("-") {
                        // replace our closing token (-) with the HTML one (/)
                        token.html = token.html.replacen("-", "/", 1);
                    }

                    out.push_str(&format!(
                        "crml_rendered.push_str(&format!(\"{}\"));//line: {}\n",
                        if last_tag.starts_with(RAW_BLOCK_TAG_PREFIX) {
                            // we need to use the raw HTML value and NOT the escaped one
                            // elements starting with RAW_BLOCK_TAG_PREFIX are special and shouldn't *actually*
                            // be rendered the page... this is an alternative to lines starting
                            // with "@"
                            token.raw.replace('"', "\\\"")
                        } else {
                            token.html.replace('"', "\\\"")
                        },
                        token.line
                    ));
                }
            }
        }

        format!(
            "{out}\nfor stack_item_ in crml_templ_stack {{
    crml_rendered.push_str(&stack_item_);
}}\ncrml_rendered\n"
        )
    }
}
