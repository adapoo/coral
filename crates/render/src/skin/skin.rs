use image::{DynamicImage, RgbaImage};

use super::{Result, SkinError};

pub struct Skin {
    pub(crate) texture: RgbaImage,
    pub(crate) slim: bool,
}

impl Skin {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let image = image::load_from_memory(data)?;
        Self::from_dynamic(image)
    }

    pub fn from_image(image: RgbaImage) -> Result<Self> {
        Self::validate_and_create(image)
    }

    pub fn from_dynamic(image: DynamicImage) -> Result<Self> {
        Self::validate_and_create(image.into_rgba8())
    }

    fn validate_and_create(texture: RgbaImage) -> Result<Self> {
        if texture.width() != 64 || texture.height() != 64 {
            return Err(SkinError::InvalidDimensions(
                texture.width(),
                texture.height(),
            ));
        }
        let slim = Self::detect_slim(&texture);
        Ok(Self { texture, slim })
    }

    fn detect_slim(texture: &RgbaImage) -> bool {
        texture.get_pixel(54, 20).0[3] == 0
    }

    pub fn is_slim(&self) -> bool {
        self.slim
    }

    pub fn texture(&self) -> &RgbaImage {
        &self.texture
    }
}
