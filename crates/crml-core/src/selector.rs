/// The extracted data from the given [`Selector].
#[derive(Clone, Debug)]
pub struct SelectorState {
    pub tag: String,
    pub classes: Option<Vec<String>>,
    pub id: Option<String>,
    pub attributes: Option<Vec<String>>,
}

impl SelectorState {
    /// Try to save the current buffer to the state (in the correct field).
    pub fn try_save(&mut self, mode: ParserMode, mut buffer: String) -> String {
        match mode {
            ParserMode::None => {
                if self.tag.is_empty() {
                    // tag cannot be overwritten
                    self.tag = buffer;
                    buffer = String::new();
                }
            }
            ParserMode::Class => {
                if self.classes.is_none() {
                    // classes is none; init classes with Some(vec![buffer])
                    self.classes = Some(vec![buffer]);
                    buffer = String::new();
                } else if let Some(ref mut classes) = self.classes {
                    // classes exists; borrow as mut ref and push buffer
                    classes.push(buffer);
                    buffer = String::new();
                }
            }
            ParserMode::Id => {
                if self.id.is_none() {
                    // id is none; set to buffer
                    self.id = Some(buffer);
                    buffer = String::new();
                }
            }
            ParserMode::Attribute => {
                if self.attributes.is_none() {
                    // attributes is none; init attributes with Some(vec![buffer])
                    self.attributes = Some(vec![buffer]);
                    buffer = String::new();
                } else if let Some(ref mut attributes) = self.attributes {
                    // attributes exists; borrow as mut ref and push buffer
                    attributes.push(buffer);
                    buffer = String::new();
                }
            }
        }

        buffer
    }

    /// Render state to HTML.
    pub fn render(self) -> String {
        let mut class_string = String::new();
        let mut id_string = String::new();
        let mut attributes_string = String::new();

        if let Some(classes) = self.classes {
            class_string = " class=\"".to_string();

            for class in classes {
                class_string.push_str(&(class + " "));
            }

            class_string += "\"";
        }

        if let Some(id) = self.id {
            id_string = format!(" id=\"{id}\"");
        }

        if let Some(attributes) = self.attributes {
            for attribute in attributes {
                attributes_string.push_str(&format!(" {attribute}"));
            }
        }

        format!("<{}{class_string}{id_string}{attributes_string}>", self.tag)
    }
}

/// The mode of the [`Selector`] parser.
#[derive(PartialEq, Eq)]
pub enum ParserMode {
    None,
    Class,
    Id,
    Attribute,
}

/// A simple parser for CSS selectors
pub struct Selector(String);

impl Selector {
    /// Create a new [`Selector`].
    pub fn new(input: String) -> Self {
        Self(input)
    }

    /// Begin parsing the selector.
    pub fn parse(self) -> SelectorState {
        let mut state = SelectorState {
            tag: String::new(),
            classes: None,
            id: None,
            attributes: None,
        };

        // parse
        let mut chars = self.0.chars();
        let mut mode: ParserMode = ParserMode::None;
        let mut buffer: String = String::new();

        while let Some(char) = chars.next() {
            match char {
                '.' => {
                    buffer = state.try_save(mode, buffer.clone());
                    mode = ParserMode::Class
                }
                '#' => {
                    buffer = state.try_save(mode, buffer.clone());
                    mode = ParserMode::Id
                }
                '[' => {
                    buffer = state.try_save(mode, buffer.clone());
                    mode = ParserMode::Attribute
                }
                ']' => {
                    buffer = state.try_save(mode, buffer.clone());
                    mode = ParserMode::None
                }
                _ => buffer.push(char),
            }
        }

        // return
        state.try_save(mode, buffer.clone()); // one last save to catch the ending stuff
        state
    }
}
