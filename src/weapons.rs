use nalgebra::{Vector2, Point2, distance};
use ncollide::query::Ray;
use ncollide::query::RayCast;
use sdl2::pixels::Color;
use ord_subset::OrdSubsetIterExt;

use Controls;
use context::Context;
use resources::Image;
use colours::{RED, GREEN};
use ships::Ship;

#[derive(Copy, Clone)]
pub enum WeaponType {
    Laser,
    TractionBeam
}

impl WeaponType {
    fn image(&self) -> Image {
        match *self {
            WeaponType::Laser => Image::Laser,
            WeaponType::TractionBeam => Image::TractionBeam
        }
    }

    fn ray_colour(&self) -> Color {
        match *self {
            WeaponType::Laser => RED,
            WeaponType::TractionBeam => GREEN
        }
    }
}

#[derive(Clone)]
pub struct Weapon {
    tag: WeaponType,
    rotation: f32,
    ray: Option<WeaponRay>
}

impl Weapon {
    pub fn new(tag: WeaponType) -> Self {
        Self {
            tag,
            rotation: 0.0,
            ray: None
        }
    }

    pub fn step(&mut self, base: Vector2<f32>, rotation: f32, controls: &Controls, rays: &mut Vec<WeaponRay>) {
        let target = (controls.mouse.1 - base.y).atan2(controls.mouse.0 - base.x) - rotation;

        let turn = (target - self.rotation).signum() * 0.1;

        if turn != 0.0 {
            if (self.rotation + turn > target) != (self.rotation > target) {
                self.rotation = target;
            } else {
                self.rotation += turn;
            }
        }

        if controls.mouse_down {
            rays.push(WeaponRay::new(self.tag, base, self.rotation + rotation));
        }
    }

    pub fn draw(&self, ctx: &mut Context, position: Vector2<f32>, rotation: f32) {
        ctx.draw_rotated(&self.tag.image(), position.x, position.y, 40.0, 40.0, (self.rotation + rotation).to_degrees());

        if let Some(ref ray) = self.ray {
            ray.draw(ctx);
        }
    }
}

#[derive(Clone)]
pub struct WeaponRay {
    tag: WeaponType,
    ray: Ray<Point2<f32>>,
    intersection: Option<Point2<f32>>
}

impl WeaponRay {
    fn new(tag: WeaponType, base: Vector2<f32>, rotation: f32) -> Self {
        Self {
            tag,
            ray: Ray::new(Point2::new(base.x, base.y), Vector2::new(rotation.cos(), rotation.sin())),
            intersection: None
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        ctx.set_colour(self.tag.ray_colour());
        let origin = self.ray.origin;
        let end = self.intersection.unwrap_or(Point2::new(origin.x + self.ray.dir.x * 2000.0, origin.y + self.ray.dir.y * 2000.0));
        ctx.draw_line(origin.x, origin.y, end.x, end.y);
    }

    pub fn intersect(&mut self, ships: &mut Vec<Ship>) {
        if let Some((ship, intersection)) = ships.iter_mut()
            .filter_map(|ship| {
                let rigid_body = ship.handle.borrow();
                rigid_body.shape().as_ref().toi_and_normal_with_ray(&rigid_body.position(), &self.ray, true)
            }.map(|intersection| (ship, self.ray.origin + self.ray.dir * intersection.toi)))
            .ord_subset_min_by_key(|&(_, intersection)| distance(&self.ray.origin, &intersection)) {
                self.intersection = Some(intersection);
        }
    }
}