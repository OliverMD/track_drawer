use seed::{prelude::*, *};

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

#[derive(Default)]
struct Model;

enum Msg {

}

fn update(msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {

}

fn view(model: &Model) -> Node<Msg> {
    div!["This is a test"]
}

fn main () {
    App::start("app", init, update, view);
}