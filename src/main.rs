use crate::Msg::LineFrom;
use rand::Rng;
use seed::{prelude::*, *};

const ENTER_KEY: &str = "Enter";

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
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
        draw_context: DrawContext::new(1000_f64 / 5_f64, 1000_f64, 2000_f64, 4, 2),
        lines: vec![],
        show_points: true,
        x_limits: (0, 4),
        y_limits: (0, 2),
        next_line: None,
    }
}

struct DrawContext {
    x_spacing: f64,
    y_spacing: f64,

    grid_width: u16,
    grid_height: u16,

    view_width: f64,
    view_height: f64,
}

impl DrawContext {
    fn new(
        spacing: f64,
        width: f64,
        height: f64,
        grid_width: u16,
        grid_height: u16,
    ) -> DrawContext {
        DrawContext {
            x_spacing: spacing,
            y_spacing: spacing,
            view_width: width,
            view_height: height,
            grid_width,
            grid_height,
        }
    }
}

struct Model {
    draw_context: DrawContext,
    lines: Vec<((i16, i16), (i16, i16))>,
    show_points: bool,
    x_limits: (i16, i16),
    y_limits: (i16, i16),
    next_line: Option<((i16, i16), (i16, i16))>,
}

enum Msg {
    ToggleShowPoints,
    NextRandomLine,
    LineFrom(u16),
    AddLine,
    NextRow,
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
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
        LineFrom(x) => {
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
    }
}

fn button_class() -> Attrs {
    C![
        "py-2 px-4 w-1/2 bg-blue-500 text-white font-semibold rounded-md shadow-md hover:bg-blue-700",
        "focus:outline-none focus:ring-2 focus:ring-green-400 focus:ring-opacity-75 m-2"
    ]
}

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["h-screen flex flex-row"],
        div![
            C!["w-1/4 bg-gray-100 h-full overflow-auto flex flex-col items-center pt-8"],
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
            button![
                "Next",
                button_class(),
                ev(Ev::Click, |_| Msg::NextRandomLine)
            ],
            button![
                "Add",
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
            div![
                C!["m-3 rounded-md bg-blue-100 bg-opacity-50 font-sans text-center text-gray-500"],
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
            C!["w-3/4 h-full flex justify-center"],
            svg![
                C!["h-full block"],
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
    ]
}

fn draw_line(
    ((from_x, from_y), (to_x, to_y)): ((i16, i16), (i16, i16)),
    ctx: &DrawContext,
) -> Vec<Node<Msg>> {
    let mut ret = Vec::new();

    let from_x = ctx.x_spacing * (from_x + 1) as f64;
    let from_y = ctx.y_spacing * (from_y + 1) as f64;
    let to_x = ctx.x_spacing * (to_x + 1) as f64;
    let to_y = ctx.y_spacing * (to_y + 1) as f64;

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
            let x_offset = ctx.x_spacing * (j as f64 + 1_f64);
            let y_offset = ctx.y_spacing * (i as f64 + 1_f64);

            ret.push(circle![
                attrs! {At::Cx => x_offset, At::Cy => y_offset, At::R => 10}
            ])
        }
    }

    ret
}

fn main() {
    App::start("app", init, update, view);
}
