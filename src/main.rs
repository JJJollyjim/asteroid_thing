extern crate sdl2;
extern crate nphysics2d;
extern crate ncollide;
extern crate nalgebra;
extern crate num_complex;
extern crate alga;
extern crate rand;
#[macro_use]
extern crate derive_is_enum_variant;
extern crate ord_subset;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use rand::Rng;

mod colours;
mod ships;
mod context;
mod resources;
mod weapons;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 800;

use ships::{Component, ComponentType, Ship, Rotation};
use weapons::WeaponType;
use context::Context;
use colours::{BLACK, WHITE};

#[derive(Default)]
pub struct Controls {
    up: bool,
    left: bool,
    right: bool,
    mouse: (f32, f32),
    mouse_down: bool
}

impl Controls {
    fn handle_key(&mut self, key: Keycode, pressed: bool) {
        match key {
            Keycode::W | Keycode::Up    => self.up    = pressed,
            Keycode::A | Keycode::Left  => self.left  = pressed,
            Keycode::D | Keycode::Right => self.right = pressed,
            _ => {}
        }
    }

    fn move_mouse(&mut self, x: i32, y: i32) {
        self.mouse = (x as f32, y as f32);
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let window = video.window("Boxes", WIDTH, HEIGHT)
        .resizable()
        .build()
        .unwrap();

    let canvas = window.into_canvas()
        .present_vsync()
        .accelerated()
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut ctx = Context::new(canvas, &texture_creator).unwrap();

    let mut rng = rand::thread_rng();
    let stars = (0 .. 1000).map(|_| (rng.gen_range(0, 1500), rng.gen_range(0, 1000))).collect::<Vec<_>>();

    let mut ships = vec![
        Ship::new(&mut ctx, vec![
            Component::new(ComponentType::Metal, -2, -2, Rotation::Up),
            Component::new(ComponentType::Pipe,  -1, -2, Rotation::Up),
            Component::new(ComponentType::Metal,  0, -2, Rotation::Up),
            Component::new(ComponentType::Pipe,   1, -2, Rotation::Up),
            Component::new(ComponentType::Metal,  2, -2, Rotation::Up),
            
            Component::new(ComponentType::Pipe,  -2, -1, Rotation::Right),
            Component::new(ComponentType::Metal, -1, -1, Rotation::Up),
            Component::new(ComponentType::Metal,  0, -1, Rotation::Up),
            Component::new(ComponentType::Metal,  1, -1, Rotation::Up),
            Component::new(ComponentType::Pipe,   2, -1, Rotation::Right),

            Component::new(ComponentType::hardpoint(WeaponType::Laser), -3, -2, Rotation::Left),
            Component::new(ComponentType::hardpoint(WeaponType::TractionBeam),  3, -2, Rotation::Right), 
            Component::new(ComponentType::Metal,  -2,  0, Rotation::Up),
            Component::new(ComponentType::Metal,  -1,  0, Rotation::Up),
            Component::new(ComponentType::Reactor, 0,  0, Rotation::Up),
            Component::new(ComponentType::Metal,   1,  0, Rotation::Up),
            Component::new(ComponentType::Metal,   2,  0, Rotation::Up),

            Component::new(ComponentType::Pipe,   -2,  1, Rotation::Right),
            Component::new(ComponentType::Metal,  -1,  1, Rotation::Up),
            Component::new(ComponentType::Metal,   0,  1, Rotation::Up),
            Component::new(ComponentType::Metal,   1,  1, Rotation::Up),
            Component::new(ComponentType::Pipe,    2,  1, Rotation::Right),

            Component::new(ComponentType::Metal,  -2,  2, Rotation::Up),
            Component::new(ComponentType::Metal,   2,  2, Rotation::Up),

            Component::new(ComponentType::Engine, -2,  3, Rotation::Up),
            Component::new(ComponentType::Engine, 2,  3, Rotation::Up)
        ], 100.0, 500.0, 0.5),
        Ship::new(&mut ctx, vec![
            Component::new(ComponentType::Rock,  0, -1, Rotation::Up),
            Component::new(ComponentType::Rock, -1,  0, Rotation::Up),
            Component::new(ComponentType::Rock,  0,  0, Rotation::Up),
            Component::new(ComponentType::Rock,  1,  0, Rotation::Up),
            Component::new(ComponentType::Rock,  0,  1, Rotation::Up),
            Component::new(ComponentType::Rock,  0,  2, Rotation::Up)
        ], 500.0, 300.0, 1.0)
    ];

    let mut controls = Controls::default();
    let mut rays = Vec::new();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                Event::KeyDown {keycode: Some(key), ..} => controls.handle_key(key, true),
                Event::KeyUp {keycode: Some(key), ..} => controls.handle_key(key, false),
                Event::MouseMotion {x, y, ..} => controls.move_mouse(x, y),
                Event::MouseButtonDown {mouse_btn: MouseButton::Left, ..} => controls.mouse_down = true,
                Event::MouseButtonUp   {mouse_btn: MouseButton::Left, ..} => controls.mouse_down = false,
                _ => {}
            }
        }

        ctx.step(1.0 / 60.0);

        ctx.set_colour(BLACK);
        ctx.clear();
        ctx.set_colour(WHITE);
        stars.iter().for_each(|&(x, y)| ctx.draw_point(x, y));

        ships.iter().for_each(|ship| ship.draw(&mut ctx));

        ships[0].step(&controls, &mut ctx, &mut rays);

        rays.iter_mut().for_each(|ray| ray.intersect(&mut ships, &mut ctx));
        rays.iter().for_each(|ray| ray.draw(&mut ctx));
        rays.clear();

        ctx.present();
    }
}
