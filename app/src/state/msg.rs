use crate::routes::ActivePage;

pub enum Msg {
    Path(ActivePage),
    Input(String, String),
}
