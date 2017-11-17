use std::error::Error;

use sdl2::video::WindowContext;
use sdl2::image::ImageRWops;
use sdl2::rwops::RWops;
use sdl2::render::{Texture, TextureCreator};

#[derive(Clone)]
pub enum Image {
    Metal,
    HorizontalPipe,
    VerticalPipe,
    Reactor,
    Engine
}

macro_rules! load_image {
    ($texture_creator: expr, $path: expr) => ({
        let rwops = RWops::from_bytes(include_bytes!(concat!("../resources/", $path)))?;
        let surface = rwops.load_png()?;
        $texture_creator.create_texture_from_surface(surface)?
    })
}

pub struct Resources<'a> {
    metal: Texture<'a>,
    horizontal_pipe: Texture<'a>,
    vertical_pipe: Texture<'a>,
    reactor: Texture<'a>,
    engine: Texture<'a>
}

impl<'a> Resources<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, Box<Error>> {
        Ok(Self {
            metal:           load_image!(texture_creator, "metal.png"),
            horizontal_pipe: load_image!(texture_creator, "horizontal_pipe.png"),
            vertical_pipe:   load_image!(texture_creator, "vertical_pipe.png"),
            reactor:         load_image!(texture_creator, "reactor.png"),
            engine:          load_image!(texture_creator, "engine.png")
        })
    }

    pub fn texture(&self, image: &Image) -> &Texture<'a> {
        match *image {
            Image::Metal          => &self.metal,
            Image::HorizontalPipe => &self.horizontal_pipe,
            Image::VerticalPipe   => &self.vertical_pipe,
            Image::Reactor        => &self.reactor,
            Image::Engine         => &self.engine
        }
    }
}