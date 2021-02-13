use seed::{prelude::*, *};

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        draw_context: DrawContext::new(1000_f64 / 4_f64, 1000_f64, 2000_f64, 4, 6),
        lines: vec![((1, 0), (3, 4))],
        show_points: true,
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
    lines: Vec<((u16, u16), (u16, u16))>,
    show_points: bool,
}

enum Msg {
    ToggleShowPoints,
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ToggleShowPoints => model.show_points = !model.show_points,
    }
}

fn view(model: &Model) -> Node<Msg> {
    log!("is checked: {}", model.show_points);
    div![
        div![
            style! {
                St::Display => "flex",
                St::JustifyContent => "center",
                St::AlignItems => "center",
                St::FlexDirection => "column"
            },
            "Controls",
            div![
                label!["Show points"],
                input![
                    id!["show-points"],
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
            style! {
                St::Width => vw(100),
                St::Height => vh(100),
                St::Display => "flex",
                St::JustifyContent => "center",
                St::AlignItems => "center",
            },
            svg![
                attrs! {
                    At::Width => percent(60),
                    At::Height => percent(60),
                    At::ViewBox => format!("0 0 {} {}", model.draw_context.view_width, model.draw_context.view_height),
                    At::PreserveAspectRatio => "xMidYMid meet"
                },
                IF!(model.show_points => gen_circles(&model.draw_context)),
                model
                    .lines
                    .iter()
                    .map(|coords| draw_line(*coords, &model.draw_context))
            ]
        ]
    ]
}

fn draw_line(
    ((from_x, from_y), (to_x, to_y)): ((u16, u16), (u16, u16)),
    ctx: &DrawContext,
) -> Node<Msg> {
    let from_x = ctx.x_spacing * (from_x + 1) as f64;
    let from_y = ctx.y_spacing * (from_y + 1) as f64;
    let to_x = ctx.x_spacing * (to_x + 1) as f64;
    let to_y = ctx.y_spacing * (to_y + 1) as f64;

    line_![
        attrs! {At::X1 => from_x, At::Y1 => from_y, At::X2 => to_x, At::Y2 => to_y, At::Stroke => "black", At::StrokeWidth => 20}
    ]
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
