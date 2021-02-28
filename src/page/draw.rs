use crate::page::draw::Msg::LineFrom;
use js_sys::Array;
use rand::Rng;
use seed::Attrs;
use seed::{prelude::*, *};
use web_sys::{Blob, BlobPropertyBag, HtmlInputElement, XmlSerializer};

const ENTER_KEY: &str = "Enter";

struct DrawContext {
    grid_width: u16,
    grid_height: u16,

    view_width: f64,
    view_height: f64,
}

impl DrawContext {
    fn new(width: f64, height: f64, grid_width: u16, grid_height: u16) -> DrawContext {
        DrawContext {
            view_width: width,
            view_height: height,
            grid_width,
            grid_height,
        }
    }

    fn x_spacing(&self) -> f64 {
        self.view_width / (self.grid_width + 1) as f64
    }

    fn y_spacing(&self) -> f64 {
        self.view_width / (self.grid_width + 1) as f64
    }
}

pub struct Model {
    draw_context: DrawContext,
    lines: Vec<((i16, i16), (i16, i16))>,
    show_points: bool,
    x_limits: (i16, i16),
    y_limits: (i16, i16),
    next_line: Option<((i16, i16), (i16, i16))>,

    svg_ref: ElRef<web_sys::HtmlElement>,
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
}

pub fn init(orders: &mut impl Orders<Msg>) -> Model {
    orders.stream(streams::window_event(Ev::KeyDown, |ev| {
        let ev: web_sys::KeyboardEvent = ev.unchecked_into();
        if ev.key() == ENTER_KEY {
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
        draw_context: DrawContext::new(1000_f64, 2000_f64, 4, 2),
        lines: vec![],
        show_points: true,
        x_limits: (0, 4),
        y_limits: (0, 2),
        next_line: None,
        svg_ref: ElRef::new(),
    }
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    let mut rng = rand::thread_rng();
    match msg {
        Msg::ToggleShowPoints => model.show_points = !model.show_points,
        Msg::NextRandomLine => {
            model.next_line = Some((
                (
                    rng.gen_range(model.x_limits.0..model.x_limits.1),
                    rng.gen_range(model.y_limits.0..model.y_limits.1),
                ),
                (
                    rng.gen_range(model.x_limits.0..model.x_limits.1),
                    rng.gen_range(model.y_limits.0..model.y_limits.1),
                ),
            ))
        }
        Msg::AddLine => {
            if let Some(line) = model.next_line {
                model.lines.push(line);
                model.next_line = None;
            }
        }
        Msg::NextRow => {
            model.draw_context.grid_height += 1;
            model.y_limits = (model.y_limits.0 + 1, model.y_limits.1 + 1);
        }
        Msg::LineFrom(x) => {
            if x <= model.draw_context.grid_width {
                model.next_line = Some((
                    ((x as i16) - 1, model.y_limits.0),
                    (
                        rng.gen_range(model.x_limits.0..model.x_limits.1),
                        rng.gen_range(model.y_limits.0..model.y_limits.1),
                    ),
                ))
            }
        }
        Msg::Download => {
            let svg_buf = XmlSerializer::new()
                .unwrap()
                .serialize_to_string(&model.svg_ref.shared_node_ws.clone_inner().unwrap())
                .unwrap();

            let mut blob_type = BlobPropertyBag::new();
            blob_type.type_("image/svg+xml;charset=utf-8");

            let arr = Array::new_with_length(1);
            arr.set(0, JsValue::from_str(&svg_buf));

            let blob =
                Blob::new_with_str_sequence_and_options(&JsValue::from(arr), &blob_type).unwrap();
            let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
            let document = web_sys::window().unwrap().document().unwrap();
            let elem = document.create_element("a").unwrap();
            elem.set_attribute("href", &url).unwrap();
            elem.set_attribute("download", "Track Image").unwrap();
            let event = document.create_event("MouseEvents").unwrap();
            event.init_event("click");
            document.body().unwrap().append_with_node_1(&elem).unwrap();
            elem.dispatch_event(&event).unwrap();
            document.body().unwrap().remove_child(&elem).unwrap();
        }
        Msg::ChangeNumCols(x) => {
            model.draw_context.grid_width = x;
            model.x_limits = (0, x as i16);
        }
        Msg::Clear => {
            model.lines.clear();
        }
    }
}

fn button_class() -> Attrs {
    C![
        "py-2 px-4 w-1/2 bg-blue-500 text-white font-semibold rounded-md shadow-md hover:bg-blue-700",
        "focus:outline-none focus:ring-2 focus:ring-green-400 focus:ring-opacity-75 m-2"
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
        C!["w-1/4 bg-gray-100 overflow-auto flex-grow-0 flex flex-col items-center pt-8 divide-y-2 px-2"],
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
                    At::Checked => model.show_points.as_at_value()
                    },
                    ev(Ev::Click, |_| Msg::ToggleShowPoints)
                ]
            ]
        ],
        div![
            C!["items-center flex flex-col w-full py-2"],
            button![
                "Next Line",
                button_class(),
                ev(Ev::Click, |_| Msg::NextRandomLine)
            ],
            button![
                "Confirm Line",
                button_class(),
                attrs! {At::Disabled => model.next_line.is_none().as_at_value()},
                ev(Ev::Click, |_| Msg::AddLine),
            ],
            button![
                "Next Row",
                button_class(),
                attrs! {At::Disabled => model.next_line.is_some().as_at_value()},
                ev(Ev::Click, |_| Msg::NextRow),
            ],
            button![
                "Clear All",
                button_class(),
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
                    dt!["Enter"],
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
                At::Value => model.x_limits.1
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
            button!["Download", button_class(), ev(Ev::Click, |_| Msg::Download)]
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
                At::ViewBox => format!("0 0 {} {}", model.draw_context.view_width, model.draw_context.view_height),
                At::PreserveAspectRatio => "xMidYMid meet",
            },
            IF!(model.show_points => gen_circles(&model.draw_context)),
            model
                .lines
                .iter()
                .map(|coords| draw_line(*coords, &model.draw_context)),
            model
                .next_line
                .map(|line| draw_line(line, &model.draw_context))
        ]
    ]
}

fn draw_line(
    ((from_x, from_y), (to_x, to_y)): ((i16, i16), (i16, i16)),
    ctx: &DrawContext,
) -> Vec<Node<Msg>> {
    let mut ret = Vec::new();

    let from_x = ctx.x_spacing() * (from_x + 1) as f64;
    let from_y = ctx.y_spacing() * (from_y + 1) as f64;
    let to_x = ctx.x_spacing() * (to_x + 1) as f64;
    let to_y = ctx.y_spacing() * (to_y + 1) as f64;

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

fn gen_circles(ctx: &DrawContext) -> Vec<Node<Msg>> {
    let mut ret = Vec::new();

    for i in 0..ctx.grid_height {
        for j in 0..ctx.grid_width {
            let x_offset = ctx.x_spacing() * (j as f64 + 1_f64);
            let y_offset = ctx.y_spacing() * (i as f64 + 1_f64);

            ret.push(circle![
                attrs! {At::Cx => x_offset, At::Cy => y_offset, At::R => 10}
            ])
        }
    }

    ret
}