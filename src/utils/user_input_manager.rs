// This object is copied from the Jelly project, maybe this can be its own crate?

// Handle the input textbox in the TUI, e.g. backspace, arrow keys, ...
pub struct UserInputManager {
    pub user_input: String,
    user_input_history: Vec<String>,
    user_input_history_index: usize,
    pub cursor_position: usize,
}

impl UserInputManager {
    pub fn new() -> Self {
        Self {
            user_input: String::new(),
            user_input_history: vec![],
            user_input_history_index: 0,
            cursor_position: 0,
        }
    }

    pub fn _insert_string(&mut self, string: &str) {
        self.user_input.push_str(string);
        self.cursor_position += string.len();
    }

    pub fn insert_char(&mut self, chr: char) {
        self.user_input.insert(self.cursor_position, chr);
        self.cursor_position += 1;
    }

    pub fn remove_char(&mut self) {
        if self.cursor_position > 0 && self.cursor_position <= self.user_input.len() {
            self.cursor_position = self.cursor_position.saturating_sub(1);
            self.user_input.remove(self.cursor_position);
        }
    }

    pub const fn move_cursor_left(&mut self) {
        self.cursor_position = self.cursor_position.saturating_sub(1);
    }

    pub const fn move_cursor_right(&mut self) {
        if self.cursor_position < self.user_input.len() {
            self.cursor_position += 1;
        }
    }

    pub fn set_to_previous_input(&mut self) {
        if self.user_input_history_index > 0 {
            self.user_input_history_index -= 1;
            self.user_input = self.user_input_history[self.user_input_history_index].clone();
            self.cursor_position = self.user_input.len();
        }
    }

    pub fn set_to_next_input(&mut self) {
        if self.user_input_history_index < self.user_input_history.len() {
            self.user_input_history_index += 1;
            if self.user_input_history_index == self.user_input_history.len() {
                self.user_input.clear();
                self.cursor_position = 0;
            } else {
                self.user_input = self.user_input_history[self.user_input_history_index].clone();
                self.cursor_position = self.user_input.len();
            }
        }
    }

    pub fn finish_current_input(&mut self) -> Option<String> {
        let result;
        // We don't want to store empty inputs
        if self.user_input.is_empty() {
            result = Some("\n".to_owned());
        } else {
            // nor the same command multiple times
            let last_input_equals_current = self
                .user_input_history
                .last()
                .is_some_and(|input| *input == self.user_input);
            if !last_input_equals_current {
                self.user_input_history
                    .push(self.user_input.clone().trim_end().to_owned());
            }

            if !self.user_input.ends_with('\n') {
                self.user_input.push('\n');
            }

            result = Some(self.user_input.clone());
            self.user_input.clear();
            self.cursor_position = 0;
        }
        // This has to be done even if the input is empty, as the user might have scrolled back
        // and deleted all input.
        self.user_input_history_index = self.user_input_history.len();

        result
    }

    pub const fn _input_empty(&self) -> bool {
        self.user_input.is_empty()
    }
}
