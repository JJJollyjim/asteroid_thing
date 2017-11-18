use colours::{WHITE, GREEN};
use context::Context;
use ships::Ship;

pub struct Selection {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32
}

impl Selection {
    pub fn new((x1, y1): (f32, f32)) -> Selection {
        Selection{
            x1: x1,
            y1: y1,
            x2: x1,
            y2: y1
        }
    }

    pub fn update_corner(&mut self, (x2, y2): (f32, f32)) {
        self.x2 = x2;
        self.y2 = y2;
    }

    pub fn draw(&self, ctx: &mut Context, ship: &Ship) {
        ctx.set_colour(GREEN);
        for c in &ship.components {
            let (pos, _) = c.position(&ship.handle.borrow().position());
            if self.includes((pos.x, pos.y)) {
                ctx.draw_point(pos.x.round() as i32, pos.y.round() as i32);
            }
        }

        ctx.set_colour(WHITE);
        ctx.draw_line(self.x1, self.y1, self.x2, self.y1);
        ctx.draw_line(self.x1, self.y2, self.x2, self.y2);
        ctx.draw_line(self.x1, self.y1, self.x1, self.y2);
        ctx.draw_line(self.x2, self.y1, self.x2, self.y2);
    }

    pub fn includes(&self, (x, y): (f32, f32)) -> bool {
        let minx = if self.x1 < self.x2 { self.x1 } else { self.x2 };
        let maxx = if self.x1 < self.x2 { self.x2 } else { self.x1 };
        let miny = if self.y1 < self.y2 { self.y1 } else { self.y2 };
        let maxy = if self.y1 < self.y2 { self.y2 } else { self.y1 };

        minx <= x && x <= maxx &&
            miny <= y && y <= maxy
    }
}
