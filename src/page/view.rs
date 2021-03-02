use crate::drawing::Drawing;
use crate::storage::STORAGE_KEY;
use seed::{prelude::*, *};

pub struct Model {
    drawings: Vec<Drawing>,
}

pub enum Msg {}

pub fn init(orders: &mut impl Orders<Msg>) -> Model {
    Model {
        drawings: LocalStorage::get(STORAGE_KEY).unwrap_or_default(),
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {}

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["m-16 justify-center flex"],
        div![
            C!["w-1/2 flex space-x-8 justify-center flex-wrap"],
            model.drawings.iter().map(render_drawing)
        ]
    ]
}

fn render_drawing(drawing: &Drawing) -> Node<Msg> {
    div![
        C!["border border-blue-500 border-opacity-25 rounded shadow-lg mt-8"],
        svg![
            C!["h-96"],
            attrs! {
                At::ViewBox => format!("0 0 {} {}", drawing.view_width, drawing.view_height),
                At::PreserveAspectRatio => "xMidYMid meet",
            },
            drawing.draw()
        ]
    ]
}
