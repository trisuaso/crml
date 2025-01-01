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
 let a = page.a;//line: 0
 let b = 2;//line: 1
crml_rendered.push_str(&format!("<div class=\"class \" id=\"id\" attr=\"value\">"));//line: 3
crml_rendered.push_str(&format!("a is {a}, b is {b}"));//line: 4
crml_rendered.push_str(&format!("</div>"));
 if a != b {//line: 6
crml_rendered.push_str(&format!("<h1   > a is not equal to b ({a} != {b})</h1>"));//line: 7
crml_rendered.push_str(&format!("<script   >"));//line: 9
crml_rendered.push_str(&format!("alert(\"Hello, world!\");"));//line: 10
crml_rendered.push_str(&format!("function test(a, b) {{"));//line: 12
crml_rendered.push_str(&format!("return a + b;"));//line: 13
crml_rendered.push_str(&format!("}}"));//line: 14
crml_rendered.push_str(&format!("</script>"));
 };//line: 16
crml_rendered
}