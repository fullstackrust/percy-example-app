use crate::routes::ActivePage;
use crate::state::Msg;
use crate::store::Store;
use crate::views::nav_bar_view::NavBarView;
use css_rs_macro::css;
use std::cell::{Ref, RefCell};
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

fn field_name(name: &str) -> Rc<RefCell<String>> {
    Rc::new(RefCell::new(name.to_string()))
}

fn form_input(store: Rc<RefCell<Store>>, name: Rc<RefCell<String>>) -> VirtualNode {
    html! {
        <input oninput=move |event: Event| {
            let input_elem = event.target().unwrap();
            let input_elem = input_elem.dyn_into::<HtmlInputElement>().unwrap();
            let value: String = input_elem.value();
            let store: Rc<RefCell<Store>> = Rc::clone(&store);
            let name: Rc<RefCell<String>> = Rc::clone(&name);
            store.borrow_mut().msg(&Msg::Input(name.borrow().to_string(), value));
        } />
    }
}

pub fn get_field(store: Ref<Store>, name: &str) -> String {
    store
        .form()
        .get("job_name")
        .unwrap_or(&"".to_string())
        .to_owned()
}

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

        html! {
            <div>
                { nav_bar }
                <div class=WRAP_CSS>
                    <h2>Management</h2>
                    <table>
                        <tr>
                            <td><label>Job name</label></td>
                            <td>{ form_input(Rc::clone(&self.store), field_name("job_name")) }</td>
                        </tr>
                        <tr>
                            <td><label>Job description</label></td>
                            <td>{ form_input(Rc::clone(&self.store), field_name("job_desc")) }</td>
                        </tr>
                        <tr>
                            <td><label>Job assigned to</label></td>
                            <td>{ form_input(Rc::clone(&self.store), field_name("job_user")) }</td>
                        </tr>
                        <tr>
                            <td><label>Job rate</label></td>
                            <td>{ form_input(Rc::clone(&self.store), field_name("job_rate")) }</td>
                        </tr>
                    </table>
                </div>
            </div>
        }
    }
}
