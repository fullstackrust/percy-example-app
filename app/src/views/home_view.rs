use crate::state::Msg;
use crate::store::Store;
use crate::views::nav_bar_view::NavBarView;
use css_rs_macro::css;

use virtual_dom_rs::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

// Components
static USER_ROLE_SELECTOR_CSS: &'static str = css! {"
    :host {
        font-size: 30px;
        font-weight: bold;
        display: flex;
        flex-direction: row;
    }

    :host > button {
        color: blue;
        background: white;
        border: 1px solid blue;
        margin: 20px;
        line-height: 50px;
        flex: 1;
    }
"};

fn user_role_button_callback(store: Rc<RefCell<Store>>, path: String) -> Box<Fn(u32)> {
    Box::new(move |_ev: u32| {
        store.borrow_mut().msg(&Msg::Path(path.to_owned()));
    })
}

fn user_role_button_component(name: &str, cb: Box<Fn(u32)>) -> VirtualNode {
    html! {
        <button
            onclick=move |ev: u32| {
                cb(ev);
            }
        >
            { name }
        </button>
    }
}

fn user_role_selector_component(
    manager_cb: Box<Fn(u32)>,
    contractor_cb: Box<Fn(u32)>,
) -> VirtualNode {
    html! {
        <div class=USER_ROLE_SELECTOR_CSS>
            { user_role_button_component("I am a Manager", manager_cb) }
            { user_role_button_component("I am a Contractor", contractor_cb) }
        </div>
    }
}

// Page
pub struct HomeView {
    store: Rc<RefCell<Store>>,
}

impl HomeView {
    pub fn new(store: Rc<RefCell<Store>>) -> HomeView {
        HomeView { store }
    }
}

impl View for HomeView {
    fn render(&self) -> VirtualNode {
        let nav_bar = NavBarView::new("/".to_owned(), Rc::clone(&self.store)).render();

        let manager_cb =
            user_role_button_callback(Rc::clone(&self.store), "/management".to_owned());
        let contractor_cb =
            user_role_button_callback(Rc::clone(&self.store), "/contractors".to_owned());

        html! {
            <div>
                { nav_bar }
                { user_role_selector_component(manager_cb, contractor_cb) }
            </div>
        }
    }
}
