extern crate sdl2;
extern crate nphysics2d;
extern crate ncollide;
extern crate nalgebra;
extern crate rand;

use sdl2::event::Event;
use rand::Rng;

mod colours;
mod blocks;
mod context;
mod resources;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 800;

use blocks::{Component, ComponentType, Ship, Rotation};
use context::Context;
use colours::{BLACK, WHITE};

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

    let mut c = vec![
        Component::new(ComponentType::Metal,                      -2, -2),
        Component::new(ComponentType::Pipe(Rotation::Horizontal), -1, -2),
        Component::new(ComponentType::Metal,                       0, -2),
        Component::new(ComponentType::Pipe(Rotation::Horizontal),  1, -2),
        Component::new(ComponentType::Metal,                       2, -2),
        
        Component::new(ComponentType::Pipe(Rotation::Vertical),   -2, -1),
        Component::new(ComponentType::Metal,                      -1, -1),
        Component::new(ComponentType::Metal,                       0, -1),
        Component::new(ComponentType::Metal,                       1, -1),
        Component::new(ComponentType::Pipe(Rotation::Vertical),    2, -1),

        Component::new(ComponentType::Metal,                      -2,  0),
        Component::new(ComponentType::Metal,                      -1,  0),
        Component::new(ComponentType::Reactor,                     0,  0),
        Component::new(ComponentType::Metal,                       1,  0),
        Component::new(ComponentType::Metal,                       2,  0),

        Component::new(ComponentType::Pipe(Rotation::Vertical),   -2,  1),
        Component::new(ComponentType::Metal,                      -1,  1),
        Component::new(ComponentType::Metal,                       0,  1),
        Component::new(ComponentType::Metal,                       1,  1),
        Component::new(ComponentType::Pipe(Rotation::Vertical),    2,  1),

        Component::new(ComponentType::Metal,                      -2,  2),
        Component::new(ComponentType::Metal,                       2,  2),

        Component::new(ComponentType::Engine,                 -2,  3),
        //Component::new(ComponentType::Engine,                  2,  3)
    ];

    let mut running = false;
    
    let mut ship_1 = Ship::new(&mut ctx, c.clone(), 100.0, 750.0);

    c.push(Component::new(ComponentType::Engine, 2,  3));

    let mut ship_2 = Ship::new(&mut ctx, c, 100.0, 500.0);

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                Event::KeyDown {..} => running = true,
                _ => {}
            }
        }

        if running {
            ctx.step(1.0 / 60.0);
            ship_1.step();
            ship_2.step();
        }

        ctx.set_draw_colour(BLACK);
        ctx.clear();
        ctx.set_draw_colour(WHITE);
        stars.iter().for_each(|&(x, y)| ctx.draw_point(x, y));

        ship_1.draw(&mut ctx);
        ship_2.draw(&mut ctx);

        ctx.present();
    }
}