#![feature(proc_macro_hygiene)]

use router_rs::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
pub use virtual_dom_rs::VirtualNode;
use wasm_bindgen;
use wasm_bindgen::prelude::*;

pub use crate::routes::*;
pub use crate::state::*;
pub use crate::store::*;

mod actions;
pub mod api;
pub mod routes;
mod state;
mod store;
mod views;

pub struct App {
    pub store: Rc<RefCell<Store>>,
    router: Rc<Router>,
}

impl App {
    pub fn new() -> App {
        let state = State::new();
        let store = Rc::new(RefCell::new(Store::new(state)));

        let router = make_router(Rc::clone(&store));

        App { store, router }
    }

    // TODO: Just use `new(config: AppConfig)` and pass in state json Option
    pub fn from_state_json(json: &str) -> App {
        let state = State::from_json(json);
        let store = Rc::new(RefCell::new(Store::new(state)));

        let router = make_router(Rc::clone(&store));

        App { store, router }
    }
}

impl App {
    pub fn render(&self, path: String) -> VirtualNode {
        #[allow(unused_variables)] // Compiler doesn't see it inside html macro
        let store = Rc::clone(&self.store);
        self.router.view(&path).unwrap()
    }
}
