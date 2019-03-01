use crate::routes::ActivePage;
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

fn user_role_button_component(name: &str, path: &str) -> VirtualNode {
    html! {
        <button
            onclick=move |_ev: u32| {
                store.borrow_mut().msg(&Msg::Path(path.to_string()));
            }
        >
            { text!(name) }
        </button>
    }
}

fn user_role_selector_component() -> VirtualNode {
    html! {
        <div class=USER_ROLE_SELECTOR_CSS>
            { user_role_button_component("I am a Manager", "/management") }
            { user_role_button_component("I am a Contractor", "/contractors") }
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
        let nav_bar = NavBarView::new(ActivePage::Home, Rc::clone(&self.store)).render();

        let store = Rc::clone(&self.store);

        let click_count = self.store.borrow().click_count();
        let click_count = &*click_count.to_string();

        let click_component =
            html! { <strong style="font-size: 30px">{ text!(click_count) }</strong> };

        html! {
            <div>
                { nav_bar }

                { user_role_selector_component() }

                <span> The button has been clicked: { click_component } times!</span>
                    <button onclick=move|_: u8| { store.borrow_mut().msg(&Msg::Click) }>
                    Click me!
                </button>

                <div> In this time Ferris has made { text!(click_count) } new friends. </div>
            </div>
        }
    }
}
