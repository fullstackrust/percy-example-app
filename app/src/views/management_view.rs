use crate::routes::ActivePage;
use crate::state::Msg;
use crate::store::Store;
use crate::views::nav_bar_view::NavBarView;
use css_rs_macro::css;
use std::cell::RefCell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

// Components
static WRAP_CSS: &'static str = css! {"
    :host {
        padding: 20px 30px;
    }
"};

// Page
pub struct ManagementView {
    store: Rc<RefCell<Store>>,
}

impl ManagementView {
    pub fn new(store: Rc<RefCell<Store>>) -> ManagementView {
        ManagementView { store }
    }
}

impl View for ManagementView {
    fn render(&self) -> VirtualNode {
        let nav_bar = NavBarView::new(ActivePage::Management, Rc::clone(&self.store)).render();
        let job_name_field: String = self
            .store
            .borrow()
            .form()
            .get("job_name")
            .unwrap_or(&"".to_string())
            .to_owned();

        let store: Rc<RefCell<Store>> = Rc::clone(&self.store);

        html! {
            <div>
                { nav_bar }
                <div class=WRAP_CSS>
                    <h2>Management</h2>
                    <div>
                        <input
                            oninput=move |event: Event| {
                                let input_elem = event.target().unwrap();
                                let input_elem = input_elem.dyn_into::<HtmlInputElement>().unwrap();
                                let job_name_value = input_elem.value();
                                let store: Rc<RefCell<Store>> = Rc::clone(&store);
                                store
                                    .borrow_mut()
                                    .msg(&Msg::Input("job_name".to_string(), job_name_value.to_string()));
                            }
                        />
                        <div>
                            { text!(job_name_field) }
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
