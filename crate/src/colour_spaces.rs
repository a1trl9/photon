//! Image manipulation effects in HSL, LCh and HSV.

extern crate image;
extern crate rand;
use image::{GenericImageView};
use palette::{Hsl, Lch, Shade, Pixel, Saturate, Srgba, Hue, Hsv};
use crate::{PhotonImage, Rgb, helpers};
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

/// Apply gamma correction. 
// #[wasm_bindgen]
// pub fn gamma_correction(mut photon_image: &mut PhotonImage, red: f32, green: f32,  blue: f32) {
//     let img = helpers::dyn_image_from_raw(&photon_image);
//     let (width, height) = img.dimensions();
//     let mut img = img.to_rgba();

//     // Initialize gamma arrays 
//     let mut gammaR: Vec<u8> = vec![];
//     let mut gammaG: Vec<u8> = vec![];
//     let mut gammaB: Vec<u8> = vec![];

//     let MAX_VALUE_INT = 255;
//     let MAX_VALUE_FLT = 255.0;
//     let REVERSE = 1.0;

//     // Set values within gamma arrays
//     for i in 0..256 {
//         gammaR[i] = min(MAX_VALUE_INT, ((MAX_VALUE_FLT * ((i as f32 / MAX_VALUE_FLT) as u32).powf(REVERSE / red) + 0.5 ) as u8));
//         gammaG[i] = min(MAX_VALUE_INT, ((MAX_VALUE_FLT * ((i as f32 / MAX_VALUE_FLT) as u32).powf(REVERSE / green) + 0.5 ) as u8);
//         gammaB[i] = min(MAX_VALUE_INT, ((MAX_VALUE_FLT * ((i as f32 / MAX_VALUE_FLT) as u32).powf(REVERSE / blue) + 0.5 ) as u8);

//     }

//     for x in 0..width {
//         for y in 0..height {
//             let px_data = img.get_pixel(x, y).data;
            
//             let r_val = px_data[0];
//             let g_val = px_data[1];
//             let b_val = px_data[2];

//             px_data[0] = gammaR[r_val as usize];
//             px_data[1] = gammaG[g_val as usize];
//             px_data[2] = gammaB[b_val as usize];
            
//             img.put_pixel(x, y, px);
//             }
//         }
//     photon_image.raw_pixels = img.to_vec();
// }

/// Image manipulation effects in the LCh colour space
/// 
/// Effects include:
/// * **saturate** - Saturation increase.
/// * **desaturate** - Desaturate the image.
/// * **shift_hue** - Hue rotation by a specified number of degrees.
/// * **darken** - Decrease the brightness.
/// * **lighten** - Increase the brightness.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
/// * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
/// # Example
/// ```
/// // For example to increase the saturation by 10%:
/// use photon::color_spaces::lch;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// lch(&mut img, "saturate", 0.1);
/// ```
#[wasm_bindgen]
pub fn lch(mut photon_image: &mut PhotonImage, mode: &str, amt: f32) {
    let img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();
    let mut img = img.to_rgba();
    for x in 0..width {
        for y in 0..height {
            let px_data = img.get_pixel(x, y).data;
            let lch_colour: Lch = Srgba::from_raw(&px_data)
                .into_format()
                .into_linear()
                .into();

            let new_color = match mode {
                // Match a single value
                "desaturate" => lch_colour.desaturate(amt),
                "saturate" => lch_colour.saturate(amt),
                "lighten" => lch_colour.lighten(amt), 
                "darken" => lch_colour.darken(amt),
                "shift_hue" => lch_colour.shift_hue(amt * 360.0),
                _ => lch_colour.saturate(amt),
            };
            
            img.put_pixel(x, y, image::Rgba {
                data: Srgba::from_linear(new_color.into()).into_format().into_raw()
            });
            }
        }
    photon_image.raw_pixels = img.to_vec();
}

/// Image manipulation effects in the HSL colour space.
/// 
/// Effects include:
/// * **saturate** - Saturation increase.
/// * **desaturate** - Desaturate the image.
/// * **shift_hue** - Hue rotation by a specified number of degrees.
/// * **darken** - Decrease the brightness.
/// * **lighten** - Increase the brightness.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
/// * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
/// # Example
/// ```
/// // For example to increase the saturation by 10%:
/// use photon::color_spaces::hsl;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// hsl(&mut img, "saturate", 0.1);
/// ``` 
#[wasm_bindgen]
pub fn hsl(mut photon_image: &mut PhotonImage, mode: &str, amt: f32) {
    // The function logic is kept separate from other colour spaces for now, 
    // since other HSL-specific logic may be implemented here, which isn't available in other colour spaces
    let mut img = helpers::dyn_image_from_raw(&photon_image).to_rgba();
    let (width, height) = img.dimensions();
        for x in 0..width {
            for y in 0..height {
                let px_data = img.get_pixel(x, y).data;

                let colour = Srgba::from_raw(&px_data).into_format();

                let hsl_colour = Hsl::from(colour);
                
                let new_color = match mode {
                    // Match a single value
                    "desaturate" => hsl_colour.desaturate(amt),
                    "saturate" => hsl_colour.saturate(amt),
                    "lighten" => hsl_colour.lighten(amt), 
                    "darken" => hsl_colour.darken(amt),
                    "shift_hue" => hsl_colour.shift_hue(amt * 360.0),
                    _ => hsl_colour.saturate(amt),
                };

                img.put_pixel(x, y, image::Rgba {
                    data: Srgba::from_linear(new_color.into()).into_format().into_raw()
                });
            }
        }

    photon_image.raw_pixels = img.to_vec();
}

/// Image manipulation in the HSV colour space. 
/// 
/// Effects include:
/// * **saturate** - Saturation increase.
/// * **desaturate** - Desaturate the image.
/// * **shift_hue** - Hue rotation by a specified number of degrees.
/// * **darken** - Decrease the brightness.
/// * **lighten** - Increase the brightness.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
/// * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
/// 
/// # Example
/// ```
/// // For example to increase the saturation by 10%:
/// use photon::color_spaces::hsv;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// hsv(&mut img, "saturate", 0.1);
/// ```
#[wasm_bindgen]
pub fn hsv(photon_image: &mut PhotonImage, mode: &str, amt: f32) {
    let img = helpers::dyn_image_from_raw(&photon_image);
    let mut img  = img.to_rgba();

    let (width, height) = img.dimensions();

        for x in 0..width {
            for y in 0..height {
                let px_data = img.get_pixel(x, y).data;

                let color = Srgba::from_raw(&px_data).into_format();

                let hsv_colour = Hsv::from(color);

                let new_color = match mode {
                    // Match a single value
                    "desaturate" => hsv_colour.desaturate(amt),
                    "saturate" => hsv_colour.saturate(amt),
                    "lighten" => hsv_colour.lighten(amt), 
                    "darken" => hsv_colour.darken(amt),
                    "shift_hue" => hsv_colour.shift_hue(amt * 360.0),
                    _ => hsv_colour.saturate(amt),
                };

                img.put_pixel(x, y, image::Rgba {
                    data: Srgba::from_linear(new_color.into()).into_format().into_raw()
                });
            }
        }
    photon_image.raw_pixels = img.to_vec();
}

/// Shift hue by a specified number of degrees in the HSL colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `mode` - The number of degrees to shift the hue by, or hue rotate by.
/// 
/// # Example
/// ```
/// // For example to hue rotate/shift the hue by 120 degrees in the HSL colour space:
/// use photon::color_spaces::hue_rotate_hsl;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// hue_rotate_hsl(&mut img, 120);
/// ``` 
#[wasm_bindgen]
pub fn hue_rotate_hsl(img: &mut PhotonImage, degrees: f32) {
    hsl(img, "shift_hue", degrees);
}

/// Shift hue by a specified number of degrees in the HSV colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `mode` - The number of degrees to shift the hue by, or hue rotate by.
/// 
/// # Example
/// ```
/// // For example to hue rotate/shift the hue by 120 degrees in the HSV colour space:
/// use photon::color_spaces::hue_rotate_hsv;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// hue_rotate_hsv(&mut img, 120);
/// ``` 
#[wasm_bindgen]
pub fn hue_rotate_hsv(img: &mut PhotonImage, degrees: f32) {
    hsv(img, "shift_hue", degrees);
}

/// Shift hue by a specified number of degrees in the LCh colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `mode` - The number of degrees to shift the hue by, or hue rotate by.
/// 
/// # Example
/// ```
/// // For example to hue rotate/shift the hue by 120 degrees in the HSL colour space:
/// use photon::color_spaces::hue_rotate_lch;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// hue_rotate_lch(&mut img, 120);
/// ``` 
#[wasm_bindgen]
pub fn hue_rotate_lch(img: &mut PhotonImage, degrees: f32) {
    lch(img, "shift_hue", degrees)
}

/// Increase the image's saturation by converting each pixel's colour to the HSL colour space
/// and increasing the colour's saturation. 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to increase the saturation by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Increasing saturation by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to increase saturation by 10% in the HSL colour space:
/// use photon::color_spaces::saturate_hsl;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// saturate_hsl(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn saturate_hsl(img: &mut PhotonImage, level: f32) {

    return hsl(img, "saturate", level);
}

/// Increase the image's saturation in the LCh colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to increase the saturation by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Increasing saturation by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to increase saturation by 40% in the Lch colour space:
/// use photon::color_spaces::saturate_lch;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// saturate_lch(&mut img, 0.4);
/// ``` 
#[wasm_bindgen]
pub fn saturate_lch(img: &mut PhotonImage, level: f32) {
    return lch(img, "saturate", level);
}

/// Increase the image's saturation in the HSV colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level by which to increase the saturation by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Increasing saturation by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to increase saturation by 30% in the HSV colour space:
/// use photon::color_spaces::saturate_hsv;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// saturate_hsv(&mut img, 0.3);
/// ``` 
#[wasm_bindgen]
pub fn saturate_hsv(img: &mut PhotonImage, level: f32) {
    return hsv(img, "saturate", level);
}

/// Lighten an image by a specified amount in the LCh colour space.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Lightening by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to lighten an image by 10% in the LCh colour space:
/// use photon::color_spaces::lighten_lch;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// lighten_lch(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn lighten_lch(img: &mut PhotonImage, level: f32) {
    return lch(img, "lighten", level);
}

/// Lighten an image by a specified amount in the HSL colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Lightening by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to lighten an image by 10% in the HSL colour space:
/// use photon::color_spaces::lighten_hsl;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// lighten_hsl(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn lighten_hsl(img: &mut PhotonImage, level: f32) {
    return hsl(img, "lighten", level);
}

/// Lighten an image by a specified amount in the HSV colour space.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Lightening by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to lighten an image by 10% in the HSV colour space:
/// use photon::color_spaces::lighten_hsv;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// lighten_hsv(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn lighten_hsv(img: &mut PhotonImage, level: f32) {
    return hsv(img, "lighten", level);
}

/// Darken the image by a specified amount in the LCh colour space.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Darkening by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to darken an image by 10% in the LCh colour space:
/// use photon::color_spaces::darken_lch;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// darken_lch(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn darken_lch(img: &mut PhotonImage, level: f32) {
    return lch(img, "darken", level);
}

/// Darken the image by a specified amount in the HSL colour space.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Darkening by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to darken an image by 10% in the HSL colour space:
/// use photon::color_spaces::darken_hsl;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// darken_hsl(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn darken_hsl(img: &mut PhotonImage, level: f32) {
    return hsl(img, "darken", level);
}

/// Darken the image's colours by a specified amount in the HSV colour space.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Darkening by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to darken an image by 10% in the HSV colour space:
/// use photon::color_spaces::darken_hsv;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// darken_hsv(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn darken_hsv(img: &mut PhotonImage, level: f32) {
    return hsv(img, "darken", level);
}

/// Desaturate the image by a specified amount in the HSV colour space.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Desaturating by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to desaturate an image by 10% in the HSV colour space:
/// use photon::color_spaces::desaturate_hsv;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/mountains.PNG");
/// 
/// desaturate_hsv(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn desaturate_hsv(img: &mut PhotonImage, level: f32) {
    return hsv(img, "desaturate", level);
}

/// Desaturate the image by a specified amount in the HSL colour space.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Desaturating by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to desaturate an image by 10% in the LCh colour space:
/// use photon::color_spaces::desaturate_hsl;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// desaturate_hsl(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn desaturate_hsl(img: &mut PhotonImage, level: f32) {
    return hsl(img, "desaturate", level);
}

/// Desaturate the image by a specified amount in the LCh colour space.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Desaturating by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to desaturate an image by 10% in the LCh colour space:
/// use photon::color_spaces::desaturate_lch;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// desaturate_lch(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn desaturate_lch(img: &mut PhotonImage, level: f32) {
    return lch(img, "desaturate", level);
}

/// Mix image with a single color, supporting passing `opacity`.
/// The algorithm comes from Jimp. See `function mix` and `function colorFn` at following link:
/// https://github.com/oliver-moran/jimp/blob/29679faa597228ff2f20d34c5758e4d2257065a3/packages/plugin-color/src/index.js
/// Specifically, result_value = (mix_color_value - origin_value) * opacity + origin_value =
/// mix_color_value * opacity + (1 - opacity) * origin_value for each
/// of RGB channel.
///
/// # Arguments
/// * `photon_image` - A DynamicImage that contains a view into the image.
/// * `mix_color` - the color to be mixed in, presetned in RGB.
/// * `opacity` - the opacity of color when mixed to image.
/// # Example
///
/// ```
/// // For example, to mix an image with rgb (50, 255, 254) and opacity 0.4:
/// use photon::colour_spaces::mix_with_colour;
///
/// let mix_colour = Rgb{50, 255, 254};
/// mix_with_colour(photon_image, mix_colour, 0.4);
/// ```
#[wasm_bindgen]
pub fn mix_with_colour(photon_image: &mut PhotonImage, mix_colour: Rgb, opacity: f32) {
    // let img = helpers::dyn_image_from_raw(&photon_image);
    // let (_width, _height) = img.dimensions();
    let length = photon_image.width * photon_image.height;
    // let mut img = img.to_rgba();

    // cache (mix_color_value * opacity) and (1 - opacity) so we dont need to calculate them each time during loop.
    let mix_red_offset = mix_colour.r as f32 * opacity;
    let mix_green_offset = mix_colour.g as f32 * opacity;
    let mix_blue_offset = mix_colour.b as f32 * opacity;
    let factor = 1.0 - opacity;

    let mut lookup_table = vec![0; 768];
    for i in 0..256 {
        lookup_table[i] = (i as f32 * factor + mix_red_offset) as u8;
        lookup_table[i + 256] = (i as f32 * factor + mix_green_offset) as u8;
        lookup_table[i + 512] = (i as f32 * factor + mix_blue_offset) as u8;
    }

    let mut index = 0;
    for i in 0..length {
    // for x in 0.._width {
    //     for y in 0.._height {
        photon_image.raw_pixels[index] = lookup_table[photon_image.raw_pixels[index] as usize];
        photon_image.raw_pixels[index + 1] = lookup_table[photon_image.raw_pixels[index + 1] as usize + 256];
        photon_image.raw_pixels[index + 2] = lookup_table[photon_image.raw_pixels[index + 2] as usize + 512];
        index += 4;
    //        let px = img.get_pixel(x, y);
    //        let r_value = lookup_table[px.data[0] as usize];
    //        let g_value = lookup_table[px.data[1] as usize + 256];
    //        let b_value = lookup_table[px.data[2] as usize + 512];
    //        let alpha = px.data[3];
    //        img.put_pixel(x, y, image::Rgba (
    //                [r_value, g_value, b_value, alpha]
    //        ));
    //    }
    }
    //photon_image.raw_pixels = img.to_vec();
}

// #[wasm_bindgen]
// pub fn selective_color_convert(mut photon_image: &mut PhotonImage, ref_color:Rgb, new_color:Rgb, fraction: f32) {
//     let img = helpers::dyn_image_from_raw(&photon_image);
//     let (_width, _height) = img.dimensions();
//     let mut img = img.to_rgba();
//     for x in 0.._width {
//         for y in 0.._height {
//             let mut px = img.get_pixel(x, y);

//             // Reference colour to compare the current pixel's colour to
//             let lab: Lab = Srgb::new(ref_color.r as f32 / 255.0, ref_color.g as f32 / 255.0, ref_color.b as f32 / 255.0).into();
      
//             // Convert the current pixel's colour to the l*a*b colour space
//             let r_val: f32 = px.data[0] as f32 / 255.0;
//             let g_val: f32 = px.data[1] as f32 / 255.0;
//             let b_val: f32 = px.data[2] as f32 / 255.0;

//             let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into();

//             let sim = color_sim(lab, px_lab);
//             if sim > 0 && sim < 40 {
//                 let newr = (((new_color.r - ref_color.r) as f32) * fraction + new_color.r as f32) as u8;
//                 let newg = (((new_color.g - ref_color.g) as f32) * fraction + new_color.g as f32) as u8;
//                 let newb = (((new_color.b - ref_color.b) as f32) * fraction + new_color.b as f32) as u8;
            
//                 img.put_pixel(x, y, image::Rgba([newr, newg, newb, 255]));
//             }
//         }
//     }
//     photon_image.raw_pixels = img.to_vec();
// }

// pub fn correct(img: &DynamicImage, mode: &'static str, colour_space: &'static str, amt: f32) -> DynamicImage {
//     let mut img  = img.to_rgb();

//     let (width, height) = img.dimensions();

//         for x in 0..width {
//             for y in 0..height {
//                 let px_data = img.get_pixel(x, y).data;

//                 let colour_to_cspace;
//                 if colour_space == "hsv" {
//                     colour_to_cspace: Hsv = Srgb::from_raw(&px_data).into_format();
//                 }
//                 else if colour_space == "hsl" {
//                     colour_to_cspace = Hsl::from(color);
//                 }
//                 else {
//                     colour_to_cspace = Lch::from(color);
//                 }
            
//                 let new_color  = match mode {
//                     // Match a single value
//                     "desaturate" => colour_to_cspace.desaturate(amt),
//                     "saturate" => colour_to_cspace.saturate(amt),
//                     "lighten" => colour_to_cspace.lighten(amt), 
//                     "darken" => colour_to_cspace.darken(amt),
//                     _ => colour_to_cspace.saturate(amt),
//                 };

//                 img.put_pixel(x, y, image::Rgb {
//                     data: Srgb::from_linear(new_color.into()).into_format().into_raw()
//                 });
//             }
//         }

//     let dynimage = image::ImageRgb8(img);
//     return dynimage;
// }
