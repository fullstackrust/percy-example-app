use crate::routes::ActivePage;
use crate::store::Store;
use crate::views::nav_bar_view::NavBarView;
use std::cell::RefCell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;

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
            <div>
             Management page here
            </div>
        </div>
        }
    }
}
