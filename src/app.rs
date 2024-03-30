use std::{collections::HashMap, str::from_utf8};

use curl::easy::Easy;
use serde_json::{json, Value};

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
    Clearing,
}

pub enum CurrentlyEditing {
    URL,
}

pub struct App {
    pub url_input: String,
    pub current_url: String,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub requests: HashMap<String, String>,
    pub response: Option<String>,
    pub current_index: usize,
}

impl App {
    pub fn new() -> App {
        App {
            current_url: String::new(),
            url_input: String::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            requests: HashMap::new(),
            response: None,
            current_index: 0,
        }
    }

    pub fn save_request(&mut self) {
        self.do_request();

        self.url_input = String::new();
        self.currently_editing = None;
    }

    pub fn wipe_requests(&mut self) {
        self.requests.clear()
    }

    pub fn do_request(&mut self) {
        let mut dst = Vec::new();
        let mut easy = Easy::new();
        {
            easy.url(self.url_input.clone().as_str()).unwrap();
            let mut transfer = easy.transfer();
            transfer
                .write_function(|data| {
                    dst.extend_from_slice(data);
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }
        let output = from_utf8(&dst).unwrap();

        self.requests
            .insert(self.url_input.clone(), String::from(output));
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::URL => self.currently_editing = Some(CurrentlyEditing::URL),
            }
        } else {
            self.currently_editing = Some(CurrentlyEditing::URL);
        }
    }

    pub fn on_press_esc(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.currently_editing = None;
    }

    pub fn on_press_enter(&mut self) {
        if let Some(editing) = &self.currently_editing {
            match editing {
                CurrentlyEditing::URL => {
                    self.save_request();
                    self.current_screen = CurrentScreen::Main;
                }
            }
        }
    }

    pub fn on_arrow_up(&mut self) {
        if self.requests.len() > 1 && self.current_index > 0 {
            self.current_index -= 1;
        }
    }

    pub fn on_arrow_down(&mut self) {
        if self.requests.len() > 1 && self.current_index < self.requests.len() - 1 {
            self.current_index += 1;
        }
    }

    pub fn on_press_backspace(&mut self) {
        if let Some(editing) = &self.currently_editing {
            match editing {
                CurrentlyEditing::URL => {
                    self.url_input.pop();
                }
            }
        }
    }

    pub fn on_press_c(&mut self) {
        self.wipe_requests();
    }

    pub fn on_press_tab(&mut self) {
        self.toggle_editing();
    }

    pub fn on_char_press(&mut self, value: char) {
        if let Some(editing) = &self.currently_editing {
            match editing {
                CurrentlyEditing::URL => self.url_input.push(value),
            }
        }
    }
}
