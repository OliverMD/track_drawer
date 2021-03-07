use crate::drawing::Drawing;
use crate::storage::STORAGE_KEY;
use crate::{icons, utils};
use seed::{prelude::*, *};

pub struct Model {
    drawings: Vec<(Drawing, ElRef<web_sys::HtmlElement>)>,
}

pub enum Msg {
    Download(usize),
    Delete(usize),
}

pub fn init(_orders: &mut impl Orders<Msg>) -> Model {
    let mut drawings: Vec<Drawing> = LocalStorage::get(STORAGE_KEY).unwrap_or_default();

    Model {
        drawings: drawings.drain(..).map(move |d| (d, ElRef::new())).collect(),
    }
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Download(idx) => {
            if let Some((_, el_ref)) = model.drawings.get(idx) {
                utils::download_svg(el_ref);
            }
        }
        Msg::Delete(idx) => {
            model.drawings.remove(idx);
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["m-16 justify-center flex"],
        div![
            C!["w-1/2 flex space-x-8 justify-center flex-wrap"],
            model
                .drawings
                .iter()
                .enumerate()
                .map(|(idx, v)| render_drawing(idx, v))
        ]
    ]
}

fn render_drawing(
    idx: usize,
    (drawing, elref): &(Drawing, ElRef<web_sys::HtmlElement>),
) -> Node<Msg> {
    div![
        C!["border border-blue-500 border-opacity-25 rounded shadow-lg mt-8 relative"],
        svg![
            C!["h-96"],
            el_ref(elref),
            attrs! {
                At::ViewBox => format!("0 0 {} {}", drawing.view_width, drawing.view_height),
                At::PreserveAspectRatio => "xMidYMid meet",
            },
            drawing.draw(),
        ],
        div![
            C!["absolute bottom-0 right-0"],
            button![
                C![
                    "w-12 stroke-current text-blue-500 opacity-25 hover:opacity-100",
                    "focus:outline-none"
                ],
                icons::download(),
                ev(Ev::Click, move |_| Msg::Download(idx))
            ],
            button![
                C!["w-12 stroke-current text-red-500 opacity-25 hover:opacity-100 focus:outline-none"],
                icons::remove(),
                ev(Ev::Click, move |_| Msg::Delete(idx))
            ]
        ]
    ]
}
