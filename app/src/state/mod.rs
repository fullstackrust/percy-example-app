use crate::routes::{get_path, ActivePage};
use serde_json;
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

mod msg;
pub use self::msg::Msg;

#[derive(Serialize, Deserialize)]
pub struct State {
    click_count: Rc<Cell<u32>>,
    #[serde(skip)]
    listeners: Vec<Box<Fn() -> ()>>,
    path: String,
    form: HashMap<String, String>,
}

impl State {
    pub fn new(count: u32) -> State {
        State {
            path: "/".to_string(),
            click_count: Rc::new(Cell::new(count)),
            // TODO: Move this to the store.. shouldn't be storing functions in state
            // just data
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

    pub fn click_count(&self) -> u32 {
        self.click_count.get()
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn form(&self) -> &HashMap<String, String> {
        &self.form
    }
}

impl State {
    fn increment_click(&mut self) {
        self.click_count.set(self.click_count.get() + 1);
    }

    fn set_path(&mut self, page: &ActivePage) {
        self.path = get_path(page).to_string();
    }

    fn set_input(&mut self, name: String, input: String) {
        self.form.insert(name, input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_deserialize() {
        let state_json = r#"{"click_count":5,"path":"/"}"#;

        let state = State::from_json(state_json);

        assert_eq!(state.click_count(), 5);

        assert_eq!(&state.to_json(), state_json);
    }
}
