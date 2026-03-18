use image::Rgba;

use super::is_outside_rounded_rect;
use crate::canvas::color::{BOX_BACKGROUND, blend};
use crate::canvas::context::DrawContext;
use crate::canvas::shape::Shape;

pub struct RoundedRect {
    width: u32,
    height: u32,
    corner_radius: u32,
    background: Rgba<u8>,
    padding: (u32, u32),
    child: Option<Box<dyn Shape>>,
}

impl RoundedRect {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            corner_radius: 30,
            background: BOX_BACKGROUND,
            padding: (20, 20),
            child: None,
        }
    }

    pub fn corner_radius(mut self, radius: u32) -> Self {
        self.corner_radius = radius;
        self
    }

    pub fn background(mut self, color: Rgba<u8>) -> Self {
        self.background = color;
        self
    }

    pub fn padding(mut self, x: u32, y: u32) -> Self {
        self.padding = (x, y);
        self
    }

    pub fn child<S: Shape + 'static>(mut self, shape: S) -> Self {
        self.child = Some(Box::new(shape));
        self
    }
}

impl Shape for RoundedRect {
    fn draw(&self, ctx: &mut DrawContext) {
        let (cw, ch) = ctx.buffer.dimensions();
        let radius = self.corner_radius.min(self.width / 2).min(self.height / 2);

        for py in 0..self.height {
            for px in 0..self.width {
                let cx = (ctx.x + px as i32) as u32;
                let cy = (ctx.y + py as i32) as u32;

                if cx >= cw || cy >= ch {
                    continue;
                }

                if is_outside_rounded_rect(px, py, self.width, self.height, radius) {
                    continue;
                }

                let bg = *ctx.buffer.get_pixel(cx, cy);
                let blended = blend(bg, self.background);
                ctx.buffer.put_pixel(cx, cy, blended);
            }
        }

        if let Some(child) = &self.child {
            let mut child_ctx = ctx.at(self.padding.0 as i32, self.padding.1 as i32);
            child.draw(&mut child_ctx);
        }
    }

    fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
