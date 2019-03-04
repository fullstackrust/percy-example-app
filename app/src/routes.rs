use router_rs::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;
pub use virtual_dom_rs::VirtualNode;

use crate::store::*;
use crate::views::*;

pub enum ActivePage {
    Home,
    Management,
    Contractors,
    Report,
}

pub fn get_path(page: &ActivePage) -> &'static str {
    match page {
        ActivePage::Home => "/",
        ActivePage::Management => "/management",
        ActivePage::Contractors => "/contractors",
        ActivePage::Report => "/report",
    }
}

pub fn get_page(path: &str) -> ActivePage {
    match path {
        "/" => ActivePage::Home,
        "/management" => ActivePage::Management,
        "/contractors" => ActivePage::Contractors,
        "/report" => ActivePage::Report,
        &_ => ActivePage::Home,
    }
}

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

pub fn make_router(store: Rc<RefCell<Store>>) -> Router {
    let mut router = Router::default();

    router.provide(store);

    router.set_route_handlers(create_routes![
        home_route,
        contractors_route,
        management_route,
        report_route
    ]);

    router
}
