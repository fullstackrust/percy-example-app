use crate::routes::ActivePage;
use crate::store::Store;
use crate::views::nav_bar_view::NavBarView;
use std::cell::RefCell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;

pub struct ReportView {
    store: Rc<RefCell<Store>>,
}

impl ReportView {
    pub fn new(store: Rc<RefCell<Store>>) -> ReportView {
        ReportView { store }
    }
}

impl View for ReportView {
    fn render(&self) -> VirtualNode {
        let nav_bar = NavBarView::new(ActivePage::Report, Rc::clone(&self.store)).render();

        html! {
            <div>
                { nav_bar }
                <div>
                    Report page here
                </div>
            </div>
        }
    }
}
