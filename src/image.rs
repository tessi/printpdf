use crate::{ColorBits, ColorSpace};
use core::fmt;
use image::GenericImageView;
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct RawImage {
    pub pixels: RawImageData,
    pub width: usize,
    pub height: usize,
    pub data_format: RawImageFormat,
    pub tag: Vec<u8>,
}

struct RawImageU8 {
    pub pixels: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub data_format: RawImageFormat,
}

impl fmt::Debug for RawImageU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RawImageU8")
            .field("pixels", &self.pixels.len())
            .field("width", &self.width)
            .field("height", &self.height)
            .field("data_format", &self.data_format)
            .finish()
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[serde(rename_all = "lowercase")]
pub enum RawImageFormat {
    R8,
    RG8,
    RGB8,
    RGBA8,
    R16,
    RG16,
    RGB16,
    RGBA16,
    BGR8,
    BGRA8,
    RGBF32,
    RGBAF32,
}

impl RawImageFormat {
    pub fn reduce_to_rgb(&self) -> Self {
        use self::RawImageFormat::*;
        match self {
            RGBA8 => RGB8,
            RGBA16 => RGB16,
            RGBAF32 => RGBF32,
            other => *other,
        }
    }

    fn from_internal(f: &azul_core::app_resources::RawImageFormat) -> Self {
        use azul_core::app_resources::RawImageFormat;
        match f {
            RawImageFormat::R8 => crate::RawImageFormat::R8,
            RawImageFormat::RG8 => crate::RawImageFormat::RG8,
            RawImageFormat::RGB8 => crate::RawImageFormat::RGB8,
            RawImageFormat::RGBA8 => crate::RawImageFormat::RGBA8,
            RawImageFormat::R16 => crate::RawImageFormat::R16,
            RawImageFormat::RG16 => crate::RawImageFormat::RG16,
            RawImageFormat::RGB16 => crate::RawImageFormat::RGB16,
            RawImageFormat::RGBA16 => crate::RawImageFormat::RGBA16,
            RawImageFormat::BGR8 => crate::RawImageFormat::BGR8,
            RawImageFormat::BGRA8 => crate::RawImageFormat::BGRA8,
            RawImageFormat::RGBF32 => crate::RawImageFormat::RGBF32,
            RawImageFormat::RGBAF32 => crate::RawImageFormat::RGBAF32,
        }
    }

    fn into_internal(&self) -> azul_core::app_resources::RawImageFormat {
        match self {
            RawImageFormat::R8 => azul_core::app_resources::RawImageFormat::R8,
            RawImageFormat::RG8 => azul_core::app_resources::RawImageFormat::RG8,
            RawImageFormat::RGB8 => azul_core::app_resources::RawImageFormat::RGB8,
            RawImageFormat::RGBA8 => azul_core::app_resources::RawImageFormat::RGBA8,
            RawImageFormat::R16 => azul_core::app_resources::RawImageFormat::R16,
            RawImageFormat::RG16 => azul_core::app_resources::RawImageFormat::RG16,
            RawImageFormat::RGB16 => azul_core::app_resources::RawImageFormat::RGB16,
            RawImageFormat::RGBA16 => azul_core::app_resources::RawImageFormat::RGBA16,
            RawImageFormat::BGR8 => azul_core::app_resources::RawImageFormat::BGR8,
            RawImageFormat::BGRA8 => azul_core::app_resources::RawImageFormat::BGRA8,
            RawImageFormat::RGBF32 => azul_core::app_resources::RawImageFormat::RGBF32,
            RawImageFormat::RGBAF32 => azul_core::app_resources::RawImageFormat::RGBAF32,
        }
    }

    pub fn has_alpha(&self) -> bool {
        use self::RawImageFormat::*;
        matches!(self, RGBA8 | RGBA16 | RGBAF32)
    }

    pub fn get_color_bits_and_space(&self) -> (ColorBits, ColorSpace) {
        use self::RawImageFormat::*;
        match self {
            R8 => (ColorBits::Bit8, ColorSpace::Greyscale),
            RG8 => (ColorBits::Bit8, ColorSpace::GreyscaleAlpha),
            RGB8 => (ColorBits::Bit8, ColorSpace::Rgb),
            RGBA8 => (ColorBits::Bit8, ColorSpace::Rgba),
            R16 => (ColorBits::Bit16, ColorSpace::Greyscale),
            RG16 => (ColorBits::Bit16, ColorSpace::GreyscaleAlpha),
            RGB16 => (ColorBits::Bit16, ColorSpace::Rgb),
            RGBA16 => (ColorBits::Bit16, ColorSpace::Rgba),
            BGR8 => (ColorBits::Bit8, ColorSpace::Rgb),
            BGRA8 => (ColorBits::Bit8, ColorSpace::Rgba),
            RGBF32 => (ColorBits::Bit16, ColorSpace::Rgb),
            RGBAF32 => (ColorBits::Bit16, ColorSpace::Rgba),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
#[serde(tag = "tag", content = "data", rename_all = "lowercase")]
pub enum RawImageData {
    // 8-bit image data
    U8(Vec<u8>),
    // 16-bit image data
    U16(Vec<u16>),
    // HDR image data
    F32(Vec<f32>),
}

impl RawImageData {
    pub fn empty(format: RawImageFormat) -> Self {
        use self::RawImageFormat::*;
        match format {
            R8 | RG8 | RGB8 | RGBA8 | BGR8 | BGRA8 => Self::U8(Vec::new()),

            R16 | RG16 | RGB16 | RGBA16 => Self::U16(Vec::new()),

            RGBF32 | RGBAF32 => Self::F32(Vec::new()),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            RawImageData::U8(vec) => vec.is_empty(),
            RawImageData::U16(vec) => vec.is_empty(),
            RawImageData::F32(vec) => vec.is_empty(),
        }
    }
}

impl RawImage {
    /// Creates an empty `RawImage`
    pub fn empty(width: usize, height: usize, format: crate::RawImageFormat) -> Self {
        Self {
            width,
            height,
            data_format: format,
            pixels: RawImageData::empty(format),
            tag: Vec::new(),
        }
    }

    /// NOTE: depends on the enabled image formats!
    pub fn decode_from_bytes(bytes: &[u8]) -> Result<Self, String> {
        use image::DynamicImage::*;

        let im = image::guess_format(bytes).map_err(|e| e.to_string())?;
        let b_len = bytes.len();

        #[cfg(not(feature = "gif"))]
        {
            let err = format!("cannot decode image (len = {b_len} bytes): printpdf is missing feature 'gif' to decode GIF files. Please enable it or construct the RawImage manually.");
            if im == image::ImageFormat::Gif {
                return Err(err);
            }
        }

        #[cfg(not(feature = "jpeg"))]
        {
            let err = format!("cannot decode image (len = {b_len} bytes): printpdf is missing feature 'jpeg' to decode JPEG files. Please enable it or construct the RawImage manually.");
            if im == image::ImageFormat::Gif {
                return Err(err);
            }
        }

        #[cfg(not(feature = "png"))]
        {
            let err = format!("cannot decode image (len = {b_len} bytes): printpdf is missing feature 'png' to decode PNG files. Please enable it or construct the RawImage manually.");
            if im == image::ImageFormat::Png {
                return Err(err);
            }
        }

        #[cfg(not(feature = "pnm"))]
        {
            let err = format!("cannot decode image (len = {b_len} bytes): printpdf is missing feature 'pnm' to decode PNM files. Please enable it or construct the RawImage manually.");
            if im == image::ImageFormat::Pnm {
                return Err(err);
            }
        }

        #[cfg(not(feature = "tiff"))]
        {
            let err = format!("cannot decode image (len = {b_len} bytes): printpdf is missing feature 'tiff' to decode TIFF files. Please enable it or construct the RawImage manually.");
            if im == image::ImageFormat::Tiff {
                return Err(err);
            }
        }

        #[cfg(not(feature = "tiff"))]
        {
            let err = format!("cannot decode image (len = {b_len} bytes): printpdf is missing feature 'tiff' to decode TIFF files. Please enable it or construct the RawImage manually.");
            if im == image::ImageFormat::Tiff {
                return Err(err);
            }
        }

        #[cfg(not(feature = "bmp"))]
        {
            let err = format!("cannot decode image (len = {b_len} bytes): printpdf is missing feature 'bmp' to decode BMP files. Please enable it or construct the RawImage manually.");
            if im == image::ImageFormat::Bmp {
                return Err(err);
            }
        }

        #[cfg(not(feature = "ico"))]
        {
            let err = format!("cannot decode image (len = {b_len} bytes): printpdf is missing feature 'ico' to decode ICO files. Please enable it or construct the RawImage manually.");
            if im == image::ImageFormat::Ico {
                return Err(err);
            }
        }

        #[cfg(not(feature = "tga"))]
        {
            let err = format!("cannot decode image (len = {b_len} bytes): printpdf is missing feature 'tga' to decode TGA files. Please enable it or construct the RawImage manually.");
            if im == image::ImageFormat::Tga {
                return Err(err);
            }
        }

        #[cfg(not(feature = "hdr"))]
        {
            let err = format!("cannot decode image (len = {b_len} bytes): printpdf is missing feature 'hdr' to decode HDR files. Please enable it or construct the RawImage manually.");
            if im == image::ImageFormat::Hdr {
                return Err(err);
            }
        }

        #[cfg(not(feature = "dds"))]
        {
            let err = format!("cannot decode image (len = {b_len} bytes): printpdf is missing feature 'dds' to decode DDS files. Please enable it or construct the RawImage manually.");
            if im == image::ImageFormat::Dds {
                return Err(err);
            }
        }

        #[cfg(not(feature = "webp"))]
        {
            let err = format!("cannot decode image (len = {b_len} bytes): printpdf is missing feature 'webp' to decode WEBP files. Please enable it or construct the RawImage manually.");
            if im == image::ImageFormat::WebP {
                return Err(err);
            }
        }

        let im = image::ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .map_err(|e| e.to_string())?
            .decode()
            .map_err(|e| e.to_string())?;

        let (w, h) = im.dimensions();
        let ct = match im.color() {
            image::ColorType::L8 => RawImageFormat::R8,
            image::ColorType::La8 => RawImageFormat::RG8,
            image::ColorType::Rgb8 => RawImageFormat::RGB8,
            image::ColorType::Rgba8 => RawImageFormat::RGBA8,
            image::ColorType::L16 => RawImageFormat::R16,
            image::ColorType::La16 => RawImageFormat::RG16,
            image::ColorType::Rgb16 => RawImageFormat::RGB16,
            image::ColorType::Rgba16 => RawImageFormat::RGBA16,
            image::ColorType::Rgb32F => RawImageFormat::RGBF32,
            image::ColorType::Rgba32F => RawImageFormat::RGBAF32,
            _ => return Err("invalid raw image format".to_string()),
        };

        let pixels = match im {
            ImageLuma8(image_buffer) => RawImageData::U8(image_buffer.into_raw()),
            ImageLumaA8(image_buffer) => RawImageData::U8(image_buffer.into_raw()),
            ImageRgb8(image_buffer) => RawImageData::U8(image_buffer.into_raw()),
            ImageRgba8(image_buffer) => RawImageData::U8(image_buffer.into_raw()),
            ImageLuma16(image_buffer) => RawImageData::U16(image_buffer.into_raw()),
            ImageLumaA16(image_buffer) => RawImageData::U16(image_buffer.into_raw()),
            ImageRgb16(image_buffer) => RawImageData::U16(image_buffer.into_raw()),
            ImageRgba16(image_buffer) => RawImageData::U16(image_buffer.into_raw()),
            ImageRgb32F(image_buffer) => RawImageData::F32(image_buffer.into_raw()),
            ImageRgba32F(image_buffer) => RawImageData::F32(image_buffer.into_raw()),
            _ => return Err("invalid pixel format".to_string()),
        };

        Ok(RawImage {
            pixels,
            width: w as usize,
            height: h as usize,
            data_format: ct,
            tag: Vec::new(),
        })
    }

    /// Translates to an internal `RawImage`, necessary for the `<img>` component
    pub fn to_internal(&self) -> azul_core::app_resources::ImageRef {
        let invalid = azul_core::app_resources::ImageRef::null_image(
            self.width,
            self.height,
            self.data_format.into_internal(),
            self.tag.clone(),
        );

        if self.pixels.is_empty() {
            invalid
        } else {
            azul_core::app_resources::ImageRef::new_rawimage(translate_to_internal_rawimage(self))
                .unwrap_or(invalid)
        }
    }
}

pub(crate) fn image_to_stream(im: RawImage, doc: &mut lopdf::Document) -> lopdf::Stream {
    use lopdf::Object::*;

    let (rgb8, alpha) = split_rawimage_into_rgb_plus_alpha(im);
    let (bpc, cs) = rgb8.data_format.get_color_bits_and_space();
    let bbox = crate::CurTransMat::Identity;
    let interpolate = false;

    let mut dict = lopdf::Dictionary::from_iter(vec![
        ("Type", Name("XObject".into())),
        ("Subtype", Name("Image".into())),
        ("Width", Integer(rgb8.width as i64)),
        ("Height", Integer(rgb8.height as i64)),
        ("BitsPerComponent", Integer(bpc.as_integer())),
        ("ColorSpace", Name(cs.as_string().into())),
        ("Interpolate", interpolate.into()),
        (
            "BBox",
            Array(bbox.as_array().iter().copied().map(Real).collect()),
        ),
    ]);

    if let Some(alpha) = alpha {
        let smask_dict = lopdf::Dictionary::from_iter(vec![
            ("Type", Name("XObject".into())),
            ("Subtype", Name("Image".into())),
            ("Width", Integer(rgb8.width as i64)),
            ("Height", Integer(rgb8.height as i64)),
            ("Interpolate", Boolean(false)),
            ("BitsPerComponent", Integer(ColorBits::Bit8.as_integer())),
            ("ColorSpace", Name(ColorSpace::Greyscale.as_string().into())),
        ]);

        let mut stream = lopdf::Stream::new(smask_dict, alpha.pixels).with_compression(true);

        let _ = stream.compress();

        dict.set("SMask", Reference(doc.add_object(stream)));
    }

    let mut s = lopdf::Stream::new(dict, rgb8.pixels).with_compression(true);

    let _ = s.compress();

    s
}

// If the image has an alpha channel, splits the alpha channel as a separate image
// to the used in the `/Smask` dictionary
fn split_rawimage_into_rgb_plus_alpha(im: RawImage) -> (RawImageU8, Option<RawImageU8>) {
    let has_alpha = im.data_format.has_alpha();

    let (orig, alpha) = if has_alpha {
        match im.pixels {
            RawImageData::U8(vec) => crate::utils::rgba_to_rgb(vec),
            RawImageData::U16(vec) => {
                let (d, alpha) = crate::utils::rgba_to_rgb16(vec);
                (
                    crate::utils::u16vec_to_u8(d),
                    crate::utils::u16vec_to_u8(alpha),
                )
            }
            RawImageData::F32(vec) => {
                let (d, alpha) = crate::utils::rgba_to_rgbf32(vec);
                (
                    crate::utils::f32vec_to_u8(d),
                    crate::utils::f32vec_to_u8(alpha),
                )
            }
        }
    } else {
        match im.pixels {
            RawImageData::U8(vec) => (vec, Vec::new()),
            RawImageData::U16(vec) => (crate::utils::u16vec_to_u8(vec), Vec::new()),
            RawImageData::F32(vec) => (crate::utils::f32vec_to_u8(vec), Vec::new()),
        }
    };

    let orig = RawImageU8 {
        pixels: orig,
        width: im.width,
        height: im.height,
        data_format: im.data_format.reduce_to_rgb(),
    };

    let alpha_mask = if alpha.is_empty() {
        None
    } else {
        Some(RawImageU8 {
            pixels: alpha,
            width: im.width,
            height: im.height,
            data_format: RawImageFormat::R8,
        })
    };

    (orig, alpha_mask)
}

pub fn translate_to_internal_rawimage(im: &RawImage) -> azul_core::app_resources::RawImage {
    azul_core::app_resources::RawImage {
        pixels: match &im.pixels {
            RawImageData::U8(vec) => azul_core::app_resources::RawImageData::U8(vec.clone().into()),
            RawImageData::U16(vec) => {
                azul_core::app_resources::RawImageData::U16(vec.clone().into())
            }
            RawImageData::F32(vec) => {
                azul_core::app_resources::RawImageData::F32(vec.clone().into())
            }
        },
        width: im.width,
        height: im.height,
        premultiplied_alpha: false,
        data_format: im.data_format.into_internal(),
        tag: im.tag.clone().into(),
    }
}
