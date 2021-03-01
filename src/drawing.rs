use seed::Attrs;
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Drawing {
    pub grid_width: u16,
    pub grid_height: u16,

    pub view_width: f64,
    pub view_height: f64,

    pub includes_points: bool,

    lines: Vec<((i16, i16), (i16, i16))>,
}

impl Drawing {
    pub fn new() -> Drawing {
        Drawing {
            grid_width: 4,
            grid_height: 2,
            view_width: 1000_f64,
            view_height: 2000_f64,
            lines: vec![],
            includes_points: true,
        }
    }

    pub fn add_line(&mut self, from: (i16, i16), to: (i16, i16)) {
        self.lines.push((from, to));
    }

    pub fn toggle_include_points(&mut self) {
        self.includes_points = !self.includes_points;
    }

    pub fn x_spacing(&self) -> f64 {
        self.view_width / (self.grid_width + 1) as f64
    }

    pub fn y_spacing(&self) -> f64 {
        self.view_width / (self.grid_width + 1) as f64
    }

    pub fn draw<Msg>(&self) -> Vec<Node<Msg>> {
        let mut ret = if self.includes_points {
            self.gen_circles()
        } else {
            Vec::new()
        };

        self.lines
            .iter()
            .map(|coords| self.draw_line(*coords))
            .for_each(|mut node| ret.append(&mut node));

        ret
    }

    fn draw_line<Msg>(
        &self,
        ((from_x, from_y), (to_x, to_y)): ((i16, i16), (i16, i16)),
    ) -> Vec<Node<Msg>> {
        let mut ret = Vec::new();

        let from_x = self.x_spacing() * (from_x + 1) as f64;
        let from_y = self.y_spacing() * (from_y + 1) as f64;
        let to_x = self.x_spacing() * (to_x + 1) as f64;
        let to_y = self.y_spacing() * (to_y + 1) as f64;

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

    fn gen_circles<Msg>(&self) -> Vec<Node<Msg>> {
        let mut ret = Vec::new();

        for i in 0..self.grid_height {
            for j in 0..self.grid_width {
                let x_offset = self.x_spacing() * (j as f64 + 1_f64);
                let y_offset = self.y_spacing() * (i as f64 + 1_f64);

                ret.push(circle![
                    attrs! {At::Cx => x_offset, At::Cy => y_offset, At::R => 10}
                ])
            }
        }
        ret
    }
}
