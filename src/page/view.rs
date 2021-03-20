use crate::drawing::Drawing;
use crate::storage::STORAGE_KEY;
use crate::{icons, utils};
use seed::{prelude::*, *};

pub struct Model {
    drawings: Vec<(Drawing, ElRef<web_sys::HtmlElement>)>,
    selected: Option<usize>,
}

#[derive(Debug)]
pub enum Msg {
    Download(usize),
    Delete(usize),
    Select(usize),
    UnSelect,
    TogglePoints(usize),
}

pub fn init(_orders: &mut impl Orders<Msg>) -> Model {
    let mut drawings: Vec<Drawing> = LocalStorage::get(STORAGE_KEY).unwrap_or_default();

    Model {
        drawings: drawings.drain(..).map(move |d| (d, ElRef::new())).collect(),
        selected: None,
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
        Msg::Select(idx) => {
            model.selected = Some(idx);
        }
        Msg::UnSelect => {
            model.selected = None;
        }
        Msg::TogglePoints(idx) => {
            if let Some((drawing, _)) = model.drawings.get_mut(idx) {
                drawing.includes_points = !drawing.includes_points;
                let drawings: Vec<Drawing> =
                    model.drawings.iter().map(|(d, _)| (*d).clone()).collect();
                LocalStorage::insert(STORAGE_KEY, &drawings).unwrap();
            }
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["h-full flex flex-row"],
        sidebar_view(model),
        drawing_view(model),
        ev(Ev::Click, move |e| {
            e.stop_propagation();
            Msg::UnSelect
        })
    ]
}

pub fn drawing_view(model: &Model) -> Node<Msg> {
    div![
        C!["m-16 justify-center flex w-full"],
        div![
            C!["w-1/2 flex space-x-8 justify-center flex-wrap"],
            model
                .drawings
                .iter()
                .enumerate()
                .map(|(idx, v)| render_drawing(idx, v, model.selected))
        ]
    ]
}

fn sidebar_view(model: &Model) -> Option<Node<Msg>> {
    if let Some(idx) = model.selected {
        if let Some((drawing, _ref)) = model.drawings.get(idx) {
            Some(div![
                C!["w-1/5 h-full bg-gray-100 flex flex-col flex-grow-0 overflow-auto"],
                div![
                    C!["flex justify-center p-2 m-2"],
                    label![
                        C!["flex items-center"],
                        "Show points",
                        input![
                            C!["form-checkbox ml-2"],
                            attrs! {
                            At::Id => "show-points",
                            At::Type => "checkbox",
                            At::Checked => drawing.includes_points.as_at_value()
                            },
                            ev(Ev::Click, move |e| {
                                e.stop_propagation();
                                Msg::TogglePoints(idx)
                            })
                        ]
                    ]
                ],
            ])
        } else {
            None
        }
    } else {
        None
    }
}

fn render_drawing(
    idx: usize,
    (drawing, elref): &(Drawing, ElRef<web_sys::HtmlElement>),
    selected: Option<usize>,
) -> Node<Msg> {
    let is_selected = selected
        .map(|selected_idx| selected_idx == idx)
        .unwrap_or(false);

    let selected_attrs: Attrs = if is_selected {
        C!["border border-blue-500 border-opacity-25 ring-4"]
    } else {
        C!["border border-blue-500 border-opacity-25"]
    };

    div![
        C!["rounded shadow-lg mt-8 relative h-96"],
        selected_attrs,
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
        ],
        ev(Ev::Click, move |e| {
            e.stop_propagation();
            Msg::Select(idx)
        })
    ]
}
