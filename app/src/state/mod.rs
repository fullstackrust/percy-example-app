use crate::routes::{get_path, ActivePage};
use serde_json;
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

mod msg;
pub use self::msg::Msg;

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip)]
    listeners: Vec<Box<Fn() -> ()>>,
    path: String,
    form: HashMap<String, String>,
}

impl State {
    pub fn new() -> State {
        State {
            path: "/".to_string(),
            listeners: vec![],
            form: HashMap::new(),
        }
    }

    pub fn from_json(state_json: &str) -> State {
        serde_json::from_str(state_json).unwrap()
    }
}

impl State {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl State {
    pub fn subscribe(&mut self, callback: Box<Fn() -> ()>) {
        self.listeners.push(callback)
    }
}

impl State {
    pub fn msg(&mut self, msg: &Msg) {
        match msg {
            Msg::Path(page) => self.set_path(page),
            Msg::Input(name, input) => self.set_input(name.to_string(), input.to_string()),
        };

        // Whenever we update state we'll let all of our state listeners know that state was
        // updated
        for callback in self.listeners.iter() {
            callback();
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn form(&self) -> &HashMap<String, String> {
        &self.form
    }
}

impl State {
    fn set_path(&mut self, page: &ActivePage) {
        self.path = get_path(page).to_string();
    }

    fn set_input(&mut self, name: String, input: String) {
        self.form.insert(name, input);
    }
}
