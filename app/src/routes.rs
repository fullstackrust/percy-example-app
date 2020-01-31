#![feature(proc_macro_hygiene)]

use router_rs::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;
pub use virtual_dom_rs::VirtualNode;

use crate::store::*;
use crate::views::*;

#[route(path = "/")]
fn home_route(store: Provided<Rc<RefCell<Store>>>) -> VirtualNode {
    HomeView::new(Rc::clone(&store)).render()
}

#[route(path = "/contractors")]
fn contractors_route(store: Provided<Rc<RefCell<Store>>>) -> VirtualNode {
    ContractorsView::new(Rc::clone(&store)).render()
}

#[route(path = "/management")]
fn management_route(store: Provided<Rc<RefCell<Store>>>) -> VirtualNode {
    ManagementView::new(Rc::clone(&store)).render()
}

#[route(path = "/report")]
fn report_route(store: Provided<Rc<RefCell<Store>>>) -> VirtualNode {
    ReportView::new(Rc::clone(&store)).render()
}

pub fn make_router(store: Rc<RefCell<Store>>) -> Rc<Router> {
    let mut router = Router::default();

    router.provide(store);

    router.set_route_handlers(create_routes![
        home_route,
        contractors_route,
        management_route,
        report_route
    ]);

    Rc::new(router)
}
