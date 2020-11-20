use console_error_panic_hook;
use fullstackrust_percy_app;
use fullstackrust_percy_app::App;
use fullstackrust_percy_app::Msg;
use fullstackrust_percy_app::VirtualNode;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
use web_sys::Element;

#[wasm_bindgen]
pub struct Client {
    app: App,
    dom_updater: DomUpdater,
}

// Expose globals from JS for things such as request animation frame
// that web sys doesn't seem to have yet
//
// TODO: Remove this and use RAF from Rust
// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.request_animation_frame
#[wasm_bindgen]
extern "C" {
    pub type GlobalJS;

    pub static global_js: GlobalJS;

    #[wasm_bindgen(method)]
    pub fn update(this: &GlobalJS);
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_state: &str) -> Client {
        console_error_panic_hook::set_once();

        let app = App::from_state_json(initial_state);

        // TODO: Use request animation frame from web_sys
        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.request_animation_frame
        app.store.borrow_mut().subscribe(Box::new(|| {
            web_sys::console::log_1(&"Updating state".into());
            global_js.update();
        }));

        app.store.borrow_mut().set_after_route(Box::new(|new_path| {
            web_sys::window()
                .unwrap()
                .history()
                .unwrap()
                .push_state_with_url(
                    &JsValue::null(),
                    "Full Stack Rust - Percy Example App",
                    Some(new_path),
                );
        }));

        let store = Rc::clone(&app.store);

        let location = web_sys::window().unwrap().location();
        let path: String = location.pathname().unwrap() + &location.search().unwrap();
        store.borrow_mut().msg(&Msg::Path(path.clone()));

        let pop_path = path.clone();
        let on_popstate =
            move |_: web_sys::Event| store.borrow_mut().msg(&Msg::Path(pop_path.clone()));
        let on_popstate = Box::new(on_popstate) as Box<FnMut(_)>;
        let mut on_popstate = Closure::wrap(on_popstate);
        web_sys::window()
            .unwrap()
            .set_onpopstate(Some(on_popstate.as_ref().unchecked_ref()));
        on_popstate.forget();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let root_node = document
            .get_element_by_id("fullstackrust-percy-web-app")
            .unwrap();
        let dom_updater = DomUpdater::new_replace_mount(app.render(path.clone()), root_node);

        Client { app, dom_updater }
    }

    pub fn render(&mut self) {
        let location = web_sys::window().unwrap().location();
        let path: String = location.pathname().unwrap() + &location.search().unwrap();
        let vdom = self.app.render(path);
        self.dom_updater.update(vdom);
    }
}
