use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::rect::{Point, Rect};
use sdl2::render::BlendMode;
use sdl2::video::WindowContext;
use sdl2::pixels::Color;
use nphysics2d::world::World;
use nphysics2d::object::RigidBody;
use nalgebra::Translation2;

use std::rc::Rc;
use std::cell::RefCell;
use std::error::Error;

use colours::BLACK;
use resources::{Resources, Image};

pub struct Context<'a> {
    canvas: WindowCanvas,
    world: World<f32>,
    resources: Resources<'a>
}

impl<'a> Context<'a> {
    pub fn new(mut canvas: WindowCanvas, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, Box<Error>> {
        canvas.set_blend_mode(BlendMode::Blend);
        canvas.set_draw_color(BLACK);

        Ok(Self {
            resources: Resources::new(texture_creator)?,
            world: World::new(),
            canvas
        })
    }

    pub fn set_draw_colour(&mut self, colour: Color) {
        self.canvas.set_draw_color(colour);
    }

    pub fn draw_point(&mut self, x: i32, y: i32) {
        self.canvas.draw_point((x, y)).unwrap();
    }

    pub fn _draw_line(&mut self, x_1: f32, y_1: f32, x_2: f32, y_2: f32) {
        self.canvas.draw_line((x_1 as i32, y_1 as i32), (x_2 as i32, y_2 as i32)).unwrap();
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    fn to_rect(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect::new((x - width / 2.0) as i32, (y - height / 2.0) as i32, width as u32, height as u32)
    }

    pub fn _draw_rotated(&mut self, image: &Image, x: f32, y: f32, width: f32, height: f32, rotation: f32) {
        self.canvas.copy_ex(
            self.resources.texture(image),
            None, Some(Self::to_rect(x, y, width, height)),
            f64::from(rotation), None,
            false, false
        ).unwrap();
    }

    pub fn draw_rotated_around(&mut self, image: &Image, x: f32, y: f32, width: f32, height: f32, center_x: f32, center_y: f32, rotation: f32) {
        self.canvas.copy_ex(
            self.resources.texture(image),
            None, Some(Self::to_rect(x, y, width, height)),
            f64::from(rotation), Some(Point::new((center_x - x + width / 2.0) as i32, (center_y - y + height / 2.0) as i32)),
            false, false
        ).unwrap();
    }

    pub fn step(&mut self, dt: f32) {
        self.world.step(dt);
    }

    pub fn add_rigid_body(&mut self, mut body: RigidBody<f32>, x: f32, y: f32) -> Rc<RefCell<RigidBody<f32>>> {
        body.append_translation(&Translation2::new(x, y));
        self.world.add_rigid_body(body)
    }
}