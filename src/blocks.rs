use nphysics2d::object::{RigidBody, RigidBodyHandle};
use ncollide::shape::{Cuboid, Compound, ShapeHandle};
use nalgebra::{Vector2, Isometry2, UnitComplex};

use context::Context;
use resources::Image;
use colours::BLACK;


const SIZE: f32 = 32.0;
const RADIUS: f32 = SIZE / 2.0;

#[derive(Clone)]
pub enum Rotation {
    Horizontal,
    Vertical
}

#[derive(Clone)]
pub enum ComponentType {
    Metal,
    Pipe(Rotation),
    Reactor,
    Engine
}

impl ComponentType {
    fn collides(&self) -> bool {
        match *self {
            ComponentType::Pipe(_) | ComponentType::Engine => false,
            _ => true
        }
    }

    fn image(&self) -> Image {
        match *self {
            ComponentType::Metal => Image::Metal,
            ComponentType::Reactor => Image::Reactor,
            ComponentType::Engine => Image::Engine,
            ComponentType::Pipe(Rotation::Horizontal) => Image::HorizontalPipe,
            ComponentType::Pipe(Rotation::Vertical) => Image::VerticalPipe,
        }
    }

    fn is_engine(&self) -> bool {
        if let ComponentType::Engine = *self {
            true
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct Component {
    tag: ComponentType,
    x: i8,
    y: i8
}

impl Component {
    pub fn new(tag: ComponentType, x: i8, y: i8) -> Self {
        Self {
            tag, x, y
        }
    }

    fn draw_at(&self, ctx: &mut Context, x: f32, y: f32, rotation: f32) {
        ctx.draw_rotated_around(&self.tag.image(), self.x() + x, self.y() + y, SIZE, SIZE, x, y, rotation);
    }

    fn x(&self) -> f32 {
        self.x as f32 * SIZE
    }

    fn y(&self) -> f32 {
        self.y as f32 * SIZE
    }

    fn vector(&self) -> Vector2<f32> {
        Vector2::new(self.x(), self.y())
    }
}

pub struct Ship {
    components: Vec<Component>,
    handle: RigidBodyHandle<f32>
}

impl Ship {
    pub fn new(ctx: &mut Context, components: Vec<Component>, x: f32, y: f32) -> Self {
        let shape = ShapeHandle::new(Cuboid::new(Vector2::new(RADIUS, RADIUS)));

        let mut body = RigidBody::new_dynamic(
            Compound::new(components.iter()
                .filter(|component| component.tag.collides())
                .map(|component| (Isometry2::new(component.vector(), 0.0), shape.clone())
        ).collect()), 0.5, 1.0, 1.0);
        
        body.set_deactivation_threshold(None);
        
        body.append_rotation(&UnitComplex::new(1.0));

        Self {
            components,
            handle: ctx.add_rigid_body(body, x, y),
        }
    }

    pub fn step(&mut self) {
        let mut inner = self.handle.borrow_mut();
        let rotation_complex = {
            let position = inner.position();
            position.rotation.scale(1000.0)
        };

        self.components.iter()
            .filter(|component| component.tag.is_engine())
            .for_each(|component| {
                let rotation = rotation_complex.arg();
                let x = rotation.cos() * component.x() - component.y() * rotation.sin();
                let y = rotation.sin() * component.x() + component.y() * rotation.cos();

                inner.apply_impulse_wrt_point(Vector2::new(rotation_complex.im, -rotation_complex.re), Vector2::new(x, y));
            });
    }

    pub fn draw(&self, ctx: &mut Context) {
        let inner = self.handle.borrow();
        let position = inner.position();
        let (x, y) = (position.translation.vector.x, position.translation.vector.y);
        let rotation = position.rotation.arg().to_degrees();

        self.components.iter().for_each(|component| component.draw_at(ctx, x, y, rotation));

        ctx.set_draw_colour(BLACK);
        ctx.draw_point(x as i32, y as i32);
    }
}