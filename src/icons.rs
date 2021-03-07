use seed::{prelude::*, *};

// Collection of icons used from heroicons

pub fn download<Msg>() -> Node<Msg> {
    svg![
        attrs! {
            At::ViewBox => "0 0 24 24",
            At::Fill => "none",
            At::Stroke => "currentColor"
        },
        path![attrs! {
            At::StrokeLinecap => "round",
            At::StrokeLineJoin => "round",
            At::StrokeWidth => 2,
            At::D => "M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
        }]
    ]
}
