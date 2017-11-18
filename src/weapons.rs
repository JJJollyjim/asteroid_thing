use nalgebra::{Vector2, Point2, distance};
use ncollide::query::Ray;
use ncollide::query::RayCast;
use sdl2::pixels::Color;
use ord_subset::OrdSubsetIterExt;
use alga::general::Inverse;

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

        // Get the turn rate of the weapon
        let turn = (target - self.rotation).signum() * 0.1;

        if turn != 0.0 {
            // If the rotation would bring it past the target, set it to the target
            if (self.rotation + turn > target) != (self.rotation > target) {
                self.rotation = target;
            } else {
                // Else rotate it
                self.rotation += turn;
            }
        }

        // If the mouse is down, add a ray
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
        // Get the end of the ray of a point off into the distance
        let end = self.intersection.unwrap_or(origin + self.ray.dir * 2000.0);
        ctx.draw_line(origin.x, origin.y, end.x, end.y);
    }

    pub fn intersect(&mut self, ships: &mut Vec<Ship>, ctx: &mut Context) {
        if let Some((i, intersection)) = ships.iter().enumerate()
            // Filter map to rays that intersect
            .filter_map(|(i, ship)| {
                let rigid_body = ship.handle.borrow();
                rigid_body.shape().as_ref().toi_and_normal_with_ray(&rigid_body.position(), &self.ray, true)
            // Map to the index and intersection point
            }.map(|intersection| (i, self.ray.origin + self.ray.dir * intersection.toi)))
            // Find the intersection point closest to the origin of the ray
            .ord_subset_min_by_key(|&(_, intersection)| distance(&self.ray.origin, &intersection)) {
                // Set the intersection point for drawing
                self.intersection = Some(intersection);

                match self.tag {
                    // Apply a force to the ship at the intersection point
                    WeaponType::TractionBeam => {
                        let mut rigid_body = ships[i].handle.borrow_mut();
                        let relative = intersection.coords - rigid_body.position().translation.vector;
                        rigid_body.apply_impulse_wrt_point(self.ray.dir.inverse() * 1000.0, relative);
                    },
                    // Damage the ship at the intersection point and remove the ship if it has been destroyed
                    WeaponType::Laser => if ships[i].damage(intersection, ctx) {
                        ships.remove(i);
                    }
                };
        }
    }
}