use crate::routes::ActivePage;

pub enum Msg {
    Path(&'static ActivePage),
    Input(String, String),
}
