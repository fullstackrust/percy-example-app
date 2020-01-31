use crate::store::Store;
use crate::views::nav_bar_view::NavBarView;
use css_rs_macro::css;
use std::cell::RefCell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;

// Components
static WRAP_CSS: &'static str = css! {"
    :host {
        padding: 20px 30px;
    }
"};

pub fn get_field(store: Rc<RefCell<Store>>, name: Rc<RefCell<String>>) -> String {
    let name = name.borrow().to_string();
    store
        .borrow()
        .form()
        .get(&name)
        .unwrap_or(&"".to_string())
        .to_owned()
}

fn form_input(store: Rc<RefCell<Store>>, name: Rc<RefCell<String>>) -> VirtualNode {
    let input_store: Rc<RefCell<Store>> = Rc::clone(&store);
    let value_store: Rc<RefCell<Store>> = Rc::clone(&store);
    let input_name: Rc<RefCell<String>> = Rc::clone(&name);
    let value_name: Rc<RefCell<String>> = Rc::clone(&name);

    html! {
        <input
            oninput=move |event: Event| {
                let input_elem = event.target().unwrap();
                let input_elem = input_elem.dyn_into::<HtmlInputElement>().unwrap();
                let value: String = input_elem.value();
                let store: Rc<RefCell<Store>> = Rc::clone(&input_store);
                let name: Rc<RefCell<String>> = Rc::clone(&input_name);
                store.borrow_mut().msg(&Msg::Input(name.borrow().to_string(), value));
            }
            value=get_field(Rc::clone(&value_store), Rc::clone(&value_name))
        />
    }
}

fn post_job_button(store: Rc<RefCell<Store>>) -> VirtualNode {
    html! {
        <button onclick=move |_ev: Event| {
            post_job(Rc::clone(&store)).wait().unwrap();
        }>Submit</button>
    }
}

fn field_name(name: &str) -> Rc<RefCell<String>> {
    Rc::new(RefCell::new(name.to_string()))
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
        let nav_bar = NavBarView::new("/management".to_owned(), Rc::clone(&self.store)).render();

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
                    { post_job_button(Rc::clone(&self.store)) }
                </div>
            </div>
        }
    }
}
