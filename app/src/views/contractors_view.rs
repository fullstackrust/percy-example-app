use crate::store::Store;
use crate::views::nav_bar_view::ActivePage;
use crate::views::nav_bar_view::NavBarView;
use std::cell::RefCell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;

pub struct ContractorsView {
    store: Rc<RefCell<Store>>,
}

impl ContractorsView {
    pub fn new(store: Rc<RefCell<Store>>) -> ContractorsView {
        ContractorsView { store }
    }
}

impl View for ContractorsView {
    fn render(&self) -> VirtualNode {
        let nav_bar = NavBarView::new(ActivePage::Contractors, Rc::clone(&self.store)).render();

        html! {
        <div>
            { nav_bar }
            <div>
             Contractors page here
            </div>
        </div>
        }
    }
}
