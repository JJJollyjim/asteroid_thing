use nphysics2d::object::{RigidBody, RigidBodyHandle};
use ncollide::shape::{Cuboid, Compound, ShapeHandle};
use nalgebra::{Vector2, Isometry2, Rotation2};
use alga::linear::Transformation;

use context::Context;
use resources::Image;
use colours::RED;
use weapons::{Weapon, WeaponType, WeaponRay};
use Controls;

const SIZE: f32 = 32.0;
const RADIUS: f32 = SIZE / 2.0;

#[derive(Clone)]
pub enum Rotation {
    Up,
    Right,
    Down,
    Left
}

impl Rotation {
    fn to_degrees(&self) -> f32 {
        match *self {
            Rotation::Up => 0.0,
            Rotation::Right => 90.0,
            Rotation::Down => 180.0,
            Rotation::Left => 270.0
        }
    }

    fn to_radians(&self) -> f32 {
        self.to_degrees().to_radians()
    }
}

#[derive(is_enum_variant, Clone)]
pub enum ComponentType {
    Metal,
    Pipe,
    Reactor,
    Engine,
    Rock,
    Hardpoint(Weapon)
}

impl ComponentType {
    fn collides(&self) -> bool {
        match *self {
            ComponentType::Pipe | ComponentType::Engine | ComponentType::Hardpoint(_) => false,
            _ => true
        }
    }

    fn image(&self) -> Image {
        match *self {
            ComponentType::Metal => Image::Metal,
            ComponentType::Reactor => Image::Reactor,
            ComponentType::Engine => Image::Engine,
            ComponentType::Pipe => Image::Pipe,
            ComponentType::Rock => Image::Rock,
            ComponentType::Hardpoint(_) => Image::Hardpoint
        }
    }

    fn density(&self) -> f32 {
        match *self {
            ComponentType::Rock => 1.0,
            ComponentType::Pipe => 0.25,
            _ => 0.5
        }
    }

    pub fn hardpoint(tag: WeaponType) -> Self {
        ComponentType::Hardpoint(Weapon::new(tag))
    }
}

#[derive(Clone)]
pub struct Component {
    tag: ComponentType,
    rotation: Rotation,
    x: i8,
    y: i8
}

impl Component {
    pub fn new(tag: ComponentType, x: i8, y: i8, rotation: Rotation) -> Self {
        Self {
            tag, rotation, x, y
        }
    }

    fn draw_at(&self, ctx: &mut Context, base: &Isometry2<f32>) {
        let (pos, rotation) = self.position(base);
        ctx.draw_rotated(&self.tag.image(), pos.x, pos.y, SIZE, SIZE, rotation.to_degrees() + self.rotation.to_degrees());

        if let ComponentType::Hardpoint(ref weapon) = self.tag {
            weapon.draw(ctx, pos, rotation);
        }
    }

    fn vector(&self) -> Vector2<f32> {
        Vector2::new(self.x as f32 * SIZE, self.y as f32 * SIZE)
    }

    fn vector_rotated(&self, rotation: f32) -> Vector2<f32> {
        Rotation2::new(rotation).transform_vector(&self.vector())
    }

    fn position(&self, base: &Isometry2<f32>) -> (Vector2<f32>, f32) {
        let rotation = base.rotation.arg();
        (base.translation.vector + self.vector_rotated(rotation), rotation)
    }

    fn step_weapon(&mut self, base: &Isometry2<f32>, controls: &Controls, rays: &mut Vec<WeaponRay>) {
        let (pos, rotation) = self.position(base);
        if let ComponentType::Hardpoint(ref mut weapon) = self.tag {
            weapon.step(pos, rotation, controls, rays)
        }
    }
}

pub enum ThrustDirection {
    Forwards,
    Left,
    Right
}

impl ThrustDirection {
    fn direction(&self, rotation: f32) -> Vector2<f32> {
        Rotation2::new(rotation).transform_vector(& match *self {
            ThrustDirection::Forwards => Vector2::new(0.0,  -1_000.0),
            ThrustDirection::Left     => Vector2::new(-100.0, 0.0),
            ThrustDirection::Right    => Vector2::new( 100.0, 0.0)
        })
    }
}

pub struct Ship {
    components: Vec<Component>,
    pub handle: RigidBodyHandle<f32>
}

impl Ship {
    pub fn new(ctx: &mut Context, components: Vec<Component>, x: f32, y: f32, rotation: f32) -> Self {
        let shape = ShapeHandle::new(Cuboid::new(Vector2::new(RADIUS, RADIUS)));
        let density = components.iter().map(|component| component.tag.density()).sum::<f32>() / components.len() as f32;
        
        Self {
            handle: ctx.add_rigid_body(RigidBody::new_dynamic(
                Compound::new(components.iter()
                    .filter(|component| component.tag.collides())
                    .map(|component| (Isometry2::new(component.vector(), 0.0), shape.clone())
                ).collect()), density, 1.0, 1.0), x, y, rotation
            ),
            components
        }
    }

    pub fn mass(&self) -> f32 {
        self.components.iter().map(|component| component.tag.density()).sum()
    }

    pub fn thrust(&mut self, direction: ThrustDirection, ctx: &mut Context) {
        if !self.components.iter().any(|component| component.tag.is_reactor()) {
            return;
        }
        
        let mut rigid_body = self.handle.borrow_mut();
        
        let (pos, rotation) = {
            let position = rigid_body.position();
            (position.translation.vector, position.rotation.arg())
        };

        ctx.set_colour(RED);

        self.components.iter()
            .filter(|component| component.tag.is_engine())
            .for_each(|component| {
                let relative_vector = component.vector_rotated(rotation);
                let direction = direction.direction(rotation + component.rotation.to_radians());
                rigid_body.apply_impulse_wrt_point(direction, relative_vector);


                let component_pos = pos + relative_vector;
                ctx.draw_line(component_pos.x, component_pos.y, pos.x, pos.y);
                ctx.draw_line(component_pos.x + direction.x, component_pos.y + direction.y, component_pos.x, component_pos.y);
            });
    }

    pub fn step(&mut self, controls: &Controls, ctx: &mut Context, rays: &mut Vec<WeaponRay>) {
        if controls.up {
            self.thrust(ThrustDirection::Forwards, ctx);
        } else if controls.left {
            self.thrust(ThrustDirection::Left, ctx);
        } else if controls.right {
            self.thrust(ThrustDirection::Right, ctx);
        }

        let rigid_body = self.handle.borrow();
        let position = rigid_body.position();

        for mut component in &mut self.components {
            component.step_weapon(position, controls, rays);
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        let rigid_body = self.handle.borrow();
        let position = rigid_body.position();

        self.components.iter().for_each(|component| component.draw_at(ctx, position));

        let (x, y) = (position.translation.vector.x, position.translation.vector.y);
        ctx.set_colour(RED);
        ctx.draw_point(x as i32, y as i32);
    }
}