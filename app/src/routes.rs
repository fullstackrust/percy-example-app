use router_rs::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use virtual_dom_rs::prelude::*;

use crate::store::*;
use crate::views::*;

// TODO: Is there a better way to do this? To deduplicate this?

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

pub fn make_router(store: Rc<RefCell<Store>>) -> Router {
    let mut router = Router::default();

    let store_clone = Rc::clone(&store);
    let param_types = HashMap::new();
    let home_route = Route::new(
        "/",
        param_types,
        Box::new(move |_params| Box::new(HomeView::new(Rc::clone(&store_clone))) as Box<View>),
    );

    let store_clone = Rc::clone(&store);
    let param_types = HashMap::new();
    let contractors_route = Route::new(
        "/contractors",
        param_types,
        Box::new(move |_params| {
            Box::new(ContractorsView::new(Rc::clone(&store_clone))) as Box<View>
        }),
    );

    let store_clone = Rc::clone(&store);
    let param_types = HashMap::new();
    let management_route = Route::new(
        "/management",
        param_types,
        Box::new(move |_params| {
            Box::new(ManagementView::new(Rc::clone(&store_clone))) as Box<View>
        }),
    );

    let store_clone = Rc::clone(&store);
    let param_types = HashMap::new();
    let report_route = Route::new(
        "/report",
        param_types,
        Box::new(move |_params| Box::new(ReportView::new(Rc::clone(&store_clone))) as Box<View>),
    );

    router.add_route(home_route);
    router.add_route(management_route);
    router.add_route(contractors_route);
    router.add_route(report_route);

    router
}
