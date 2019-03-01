use crate::routes::ActivePage;

pub enum Msg {
    Click,
    Path(&'static ActivePage),
}
