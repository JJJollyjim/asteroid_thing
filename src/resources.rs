use std::error::Error;

use sdl2::video::WindowContext;
use sdl2::image::ImageRWops;
use sdl2::rwops::RWops;
use sdl2::render::{Texture, TextureCreator};

#[derive(Clone)]
pub enum Image {
    Metal,
    Pipe,
    Reactor,
    Engine,
    Rock,
    Hardpoint,
    Laser,
    TractionBeam
}

macro_rules! load_image {
    ($texture_creator: expr, $path: expr) => ({
        // Load the bytes into a read/write struct, create a surface and then a texture from that surface
        let rwops = RWops::from_bytes(include_bytes!(concat!("../resources/", $path)))?;
        let surface = rwops.load_png()?;
        $texture_creator.create_texture_from_surface(surface)?
    })
}

pub struct Resources<'a> {
    metal: Texture<'a>,
    pipe: Texture<'a>,
    reactor: Texture<'a>,
    engine: Texture<'a>,
    rock: Texture<'a>,
    hardpoint: Texture<'a>,
    laser: Texture<'a>,
    traction_beam: Texture<'a>
}

impl<'a> Resources<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, Box<Error>> {
        Ok(Self {
            metal:         load_image!(texture_creator, "metal.png"),
            pipe:          load_image!(texture_creator, "pipe.png"),
            reactor:       load_image!(texture_creator, "reactor.png"),
            engine:        load_image!(texture_creator, "engine.png"),
            rock:          load_image!(texture_creator, "rock.png"),
            hardpoint:     load_image!(texture_creator, "hardpoint.png"),
            laser:         load_image!(texture_creator, "laser.png"),
            traction_beam: load_image!(texture_creator, "traction_beam.png")
        })
    }

    pub fn texture(&self, image: &Image) -> &Texture<'a> {
        match *image {
            Image::Metal        => &self.metal,
            Image::Pipe         => &self.pipe,
            Image::Reactor      => &self.reactor,
            Image::Engine       => &self.engine,
            Image::Rock         => &self.rock,
            Image::Hardpoint    => &self.hardpoint,
            Image::Laser        => &self.laser,
            Image::TractionBeam => &self.traction_beam
        }
    }
}