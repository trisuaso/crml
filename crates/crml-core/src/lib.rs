pub mod generator;
pub mod selector;

use selector::Selector;

/// The type of a given [`Token`].
#[derive(Debug)]
pub enum TokenType {
    /// A direct string of Rust code:
    ///
    /// ```text
    /// <let a = 1>
    /// ```
    ///
    /// Begins with `<` and ends with `>`.
    RustString,
    /// A CSS selector which will be transformed into an HTML element:
    ///
    /// ```text
    /// %element.class#id[attr=val]
    /// ```
    ///
    /// Begins with `%`. If a single quote (`'`) comes after the selector,
    /// everything else on the line will be treated as the `innerHTML`, and the
    /// element will be closed as well.
    Selector,
    /// A closing tag
    ///
    /// ```text
    /// @element
    /// ```
    ///
    /// Begins with `@`.
    End,
    /// Raw text:
    ///
    /// ```text
    /// anything not matched into the previous types
    /// ```
    Raw,
}

/// A *token* is a representation of fully parsed data.
#[derive(Debug)]
pub struct Token {
    /// The type of the token.
    pub r#type: TokenType,
    /// The raw CRML string of the token.
    pub raw: String,
    /// The HTML string of the token.
    pub html: String,
}

impl Token {
    /// Create a [`Token`] from a given [`String`] value,
    pub fn from_string(value: String) -> Option<Self> {
        let mut chars = value.chars();
        match match chars.next() {
            Some(c) => c,
            None => {
                return Some(Self {
                    r#type: TokenType::Raw,
                    raw: "\n".to_string(),
                    html: "\n".to_string(),
                });
            }
        } {
            '<' => {
                // starting with an opening sign; rust data
                // not much real parsing to do here
                let mut raw = String::new();

                while let Some(char) = chars.next() {
                    // enforce ending char to close
                    if char == '>' {
                        break;
                    }

                    // push char
                    raw.push(char);
                }

                return Some(Self {
                    r#type: TokenType::RustString,
                    raw,
                    html: String::new(),
                });
            }
            '%' => {
                // starting with a beginning sign; selector
                let mut raw = String::new();
                let mut data = String::new();
                let mut inline: bool = false;

                while let Some(char) = chars.next() {
                    // check for inline char
                    if char == '\'' {
                        inline = true;
                        break;
                    }

                    // push char
                    raw.push(char);
                }

                if inline {
                    while let Some(char) = chars.next() {
                        data.push(char);
                    }
                }

                let selector = Selector::new(raw.clone()).parse();
                return Some(Self {
                    r#type: TokenType::Selector,
                    raw: format!("{raw}{data}"),
                    html: if inline {
                        // inline element
                        format!("{}{data}</{}>", selector.clone().render(), selector.tag)
                    } else {
                        selector.render()
                    },
                });
            }
            '@' => {
                // starting with a ending sign; end
                let mut raw = String::new();

                while let Some(char) = chars.next() {
                    raw.push(char);
                }

                return Some(Self {
                    r#type: TokenType::End,
                    raw: raw.clone(),
                    html: format!("</{raw}>"),
                });
            }
            _ => {
                // no recognizable starting character; raw data
                return Some(Self {
                    r#type: TokenType::Raw,
                    raw: value.clone(),
                    html: value,
                });
            }
        }
    }
}

/// Iterable version of [`Parser`]. Created through [`Parser::parse`].
pub struct TokenStream(Parser);

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// The current state of the given [`Parser`].
pub struct ParserState {
    /// The current line the parser is on.
    ///
    /// We parse line by line to enforce whitespace. This means we just need to
    /// track what line we are currently on.
    pub line_number: i32,
}

impl Default for ParserState {
    fn default() -> Self {
        Self { line_number: -1 }
    }
}

/// General character-by-character parser for CRML.
pub struct Parser(Vec<String>, ParserState);

impl Parser {
    /// Create a new [`Parser`]
    pub fn new(input: String) -> Self {
        let mut lines = Vec::new();

        for line in input.split("\n") {
            lines.push(line.to_owned())
        }

        // ...
        Self(lines, ParserState::default())
    }

    /// Begin parsing the `input`
    pub fn parse(self) -> TokenStream {
        TokenStream(self)
    }

    /// Parse the next line in the given `input`
    pub fn next(&mut self) -> Option<Token> {
        // get line
        self.1.line_number += 1;
        let line = match self.0.get(self.1.line_number as usize) {
            Some(l) => l,
            None => return None,
        };

        // parse token
        Token::from_string(line.trim().to_owned())
    }
}