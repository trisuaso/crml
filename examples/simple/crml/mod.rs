// This file is @generated.
mod data;
use data::*;
/// Render the `index.crml` template with the given [`TestProps`] properties.
///
/// # Arguments
/// * `page` - [`TestProps`]
///
/// # Returns
/// Rendered string.
///
/// # Example
/// ```rust
/// println!("rendered: {}", index(TestProps::default()));
/// ```
pub fn index(page: TestProps) -> String {
    let mut crml_rendered = String::new();
let a = page.a;
let b = 2;
crml_rendered.push_str(&format!("<div class=\"class \" id=\"id\" attr=\"value\">"));
crml_rendered.push_str(&format!("a is {a}, b is {b}"));
crml_rendered.push_str(&format!("</div>"));
if a != b {
crml_rendered.push_str(&format!("<h1   > a is not equal to b ({a} != {b})</h1>"));
}
crml_rendered
}