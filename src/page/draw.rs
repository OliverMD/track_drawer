use rand::Rng;
use seed::Attrs;
use seed::{prelude::*, *};
use web_sys::HtmlInputElement;

use crate::drawing::Drawing;
use crate::page::draw::Msg::LineFrom;
use crate::storage::STORAGE_KEY;
use crate::utils;

pub struct Model {
    y_limits: (i16, i16),
    next_line: Option<((i16, i16), (i16, i16))>,
    drawing: Drawing,

    svg_ref: ElRef<web_sys::HtmlElement>,

    #[allow(dead_code)]
    input_handle: StreamHandle, // Make sure we drop our stream when the user leave this page
}

pub enum Msg {
    ToggleShowPoints,
    NextRandomLine,
    LineFrom(u16),
    AddLine,
    NextRow,
    Download,
    ChangeNumCols(u16),
    Clear,
    Save,
}

pub fn init(orders: &mut impl Orders<Msg>) -> Model {
    let input_handle = orders.stream_with_handle(streams::window_event(Ev::KeyDown, |ev| {
        let ev: web_sys::KeyboardEvent = ev.unchecked_into();
        if ev.key() == "c" {
            Some(Msg::AddLine)
        } else if ev.key() == "r" {
            Some(Msg::NextRandomLine)
        } else if ev.key() == "n" {
            Some(Msg::NextRow)
        } else {
            if let Some(num) = ev.key().parse().ok() {
                Some(LineFrom(num))
            } else {
                None
            }
        }
    }));
    Model {
        y_limits: (0, 2),
        next_line: None,
        drawing: Drawing::new(),
        svg_ref: ElRef::new(),
        input_handle,
    }
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    let mut rng = rand::thread_rng();
    match msg {
        Msg::ToggleShowPoints => model.drawing.toggle_include_points(),
        Msg::NextRandomLine => {
            let x_limits = (0 as i16, model.drawing.grid_width as i16);
            model.next_line = Some((
                (
                    rng.gen_range(x_limits.0..x_limits.1),
                    rng.gen_range(model.y_limits.0..model.y_limits.1),
                ),
                (
                    rng.gen_range(x_limits.0..x_limits.1),
                    rng.gen_range(model.y_limits.0..model.y_limits.1),
                ),
            ))
        }
        Msg::AddLine => {
            if let Some((from, to)) = model.next_line {
                model.drawing.add_line(from, to);
                model.next_line = None;
            }
        }
        Msg::NextRow => {
            model.drawing.grid_height += 1;
            model.y_limits = (model.y_limits.0 + 1, model.y_limits.1 + 1);
        }
        Msg::LineFrom(x) => {
            if x <= model.drawing.grid_width {
                model.next_line = Some((
                    ((x as i16) - 1, model.y_limits.0),
                    (
                        rng.gen_range(0_i16..model.drawing.grid_width as i16),
                        rng.gen_range(model.y_limits.0..model.y_limits.1),
                    ),
                ))
            }
        }
        Msg::Download => {
            utils::download_svg(&model.svg_ref);
        }
        Msg::ChangeNumCols(x) => {
            model.drawing.grid_width = x;
        }
        Msg::Clear => {
            model.y_limits = (0, 2);
            model.drawing = Drawing::new();
        }
        Msg::Save => {
            let mut saved_drawings: Vec<Drawing> =
                LocalStorage::get(STORAGE_KEY).unwrap_or(Vec::new());
            saved_drawings.push(std::mem::replace(&mut model.drawing, Drawing::new()));
            model.next_line = None;
            LocalStorage::insert(STORAGE_KEY, &saved_drawings).expect("Saving drawing failed")
        }
    }
}

fn button_class(disabled: bool) -> Attrs {
    C![
        "py-2 px-4 w-1/2 bg-blue-500 text-white font-semibold rounded-md shadow-md",
        "focus:outline-none focus:ring-4 focus:ring-indigo-400 focus:ring-opacity-75 m-2",
        if disabled {
            "opacity-50 cursor-default"
        } else {
            "hover:bg-blue-700"
        }
    ]
}

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["flex flex-grow flex-row w-screen"],
        sidebar_view(model),
        svg_view(model)
    ]
}

fn sidebar_view(model: &Model) -> Node<Msg> {
    div![
        C!["w-1/5 bg-gray-100 overflow-auto flex-grow-0 flex flex-col items-center pt-8 divide-y-2 px-2 shadow-md"],
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
                    At::Checked => model.drawing.includes_points.as_at_value()
                    },
                    ev(Ev::Click, |_| Msg::ToggleShowPoints)
                ]
            ]
        ],
        div![
            C!["items-center flex flex-col w-full py-2"],
            button![
                "Next Line",
                button_class(false),
                ev(Ev::Click, |_| Msg::NextRandomLine)
            ],
            button![
                "Confirm Line",
                button_class(model.next_line.is_none()),
                attrs! {At::Disabled => model.next_line.is_none().as_at_value()},
                ev(Ev::Click, |_| Msg::AddLine),
            ],
            button![
                "Next Row",
                button_class(model.next_line.is_some()),
                attrs! {At::Disabled => model.next_line.is_some().as_at_value()},
                ev(Ev::Click, |_| Msg::NextRow),
            ],
            button![
                "Clear All",
                button_class(false),
                ev(Ev::Click, |_| Msg::Clear)
            ],
        ],
        div![
        C!["py-2"],
            div![
                C!["m-3 p-2 rounded-md bg-blue-100 bg-opacity-50 font-sans text-center text-gray-500 shadow"],
                "Keyboard Shortcuts",
                dl![
                    C!["grid grid-cols-2 gap-2 text-sm my-3 font-light"],
                    dt!["r"],
                    dd!["Random line"],
                    dt!["n"],
                    dd!["New row"],
                    dt!["c"],
                    dd!["Adds the last random line"],
                    dt!["0..9"],
                    dd!["Random line from numbered point"]
                ]
            ]
        ],
        div![
            C!["p-2 w-full flex flex-col items-center my-2"],
            label![
                C!["text-left mb-1 w-full"],
                "Grid width:"
            ],
            input![
                C!["w-full"],
                attrs!{
                At::Type => "range",
                At::Min => 1,
                At::Max => 8,
                At::Step => 1,
                At::Value => model.drawing.grid_width
                },
                ev(Ev::Change, |change| {
                    let input_elem: HtmlInputElement = change.target().unwrap().dyn_into().unwrap();
                    Msg::ChangeNumCols(input_elem.value().parse().unwrap())
                })
            ],
            div![
                C!["flex justify-between mt-2 text-xs text-gray-600 w-full px-1"],
                span![C!["text-left"], format!("{}", 1)],
                (2..8)
                    .into_iter()
                    .map(|i| span![C!["text-center left-2"], format!("{}", i)]),
                span![C!["text-right"], format!("{}", 8)]
            ]
        ],
        div![
            C!["pt-2 items-center flex flex-col w-full"],
            button!["Save", button_class(false), ev(Ev::Click, |_| Msg::Save)],
            button!["Download", button_class(false), ev(Ev::Click, |_| Msg::Download)]
        ]
    ]
}

fn svg_view(model: &Model) -> Node<Msg> {
    div![
        C!["w-3/4 flex flex-grow justify-center w-full"],
        style! {
            St::Height => "96vh"
        },
        svg![
            C!["w-full h-full"],
            el_ref(&model.svg_ref),
            attrs! {
                At::ViewBox => format!("0 0 {} {}", model.drawing.view_width, model.drawing.view_height),
                At::PreserveAspectRatio => "xMidYMid meet",
            },
            model.drawing.draw(),
            model.next_line.map(|line| draw_line(
                line,
                model.drawing.x_spacing(),
                model.drawing.y_spacing()
            ))
        ]
    ]
}

fn draw_line(
    ((from_x, from_y), (to_x, to_y)): ((i16, i16), (i16, i16)),
    x_spacing: f64,
    y_spacing: f64,
) -> Vec<Node<Msg>> {
    let mut ret = Vec::new();

    let from_x = x_spacing * (from_x + 1) as f64;
    let from_y = y_spacing * (from_y + 1) as f64;
    let to_x = x_spacing * (to_x + 1) as f64;
    let to_y = y_spacing * (to_y + 1) as f64;

    ret.push(line_![
        attrs! {At::X1 => from_x, At::Y1 => from_y, At::X2 => to_x, At::Y2 => to_y, At::Stroke => "black", At::StrokeWidth => 20}
    ]);
    ret.push(circle![
        attrs! {At::Cx => from_x, At::Cy => from_y, At::R => 10}
    ]);
    ret.push(circle![
        attrs! {At::Cx => to_x, At::Cy => to_y, At::R => 10}
    ]);

    ret
}
