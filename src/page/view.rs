use seed::{prelude::*, *};

pub struct Model {}

pub enum Msg {}

pub fn init(orders: &mut impl Orders<Msg>) -> Model {
    Model {}
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {}

pub fn view(model: &Model) -> Node<Msg> {
    div!["View Page"]
}
