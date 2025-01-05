use image::{ImageBuffer, Rgba};

pub struct ColorStep {
    pub color: [u8; 3],
    pub step: f32,
}

pub enum Palette {
    Default,
    StarryNight,
}

pub fn get_palette(palette: Palette) -> Vec<ColorStep> {
    match palette {
        Palette::Default => vec![
            ColorStep {
                color: [0, 150, 255],
                step: 0.0,
            },
            ColorStep {
                color: [255, 165, 0],
                step: 0.5,
            },
            ColorStep {
                color: [255, 0, 0],
                step: 1.0,
            },
        ],
        Palette::StarryNight => vec![
            ColorStep {
                color: [41, 37, 29],
                step: 0.0,
            },
            ColorStep {
                color: [47, 55, 116],
                step: 0.25,
            },
            ColorStep {
                color: [77, 99, 148],
                step: 0.5,
            },
            ColorStep {
                color: [126, 164, 176],
                step: 0.75,
            },
            ColorStep {
                color: [205, 210, 127],
                step: 1.0,
            },
        ],
    }
}

fn upscale_vectors(
    u_vec: &[f32],
    v_vec: &[f32],
    grid_width: usize,
    grid_height: usize,
    upscale_factor: u32,
) -> (Vec<f32>, Vec<f32>) {
    let new_width = grid_width * upscale_factor as usize;
    let new_height = grid_height * upscale_factor as usize;
    let mut new_u_vec = vec![0.0; new_width * new_height];
    let mut new_v_vec = vec![0.0; new_width * new_height];

    for y in 0..new_height {
        for x in 0..new_width {
            // Map new grid coordinates to original grid coordinates
            let orig_x = (x as f32) / (upscale_factor as f32);
            let orig_y = (y as f32) / (upscale_factor as f32);

            // Calculate indices and interpolation weights
            let x0 = orig_x.floor() as usize;
            let x1 = (x0 + 1).min(grid_width - 1);
            let y0 = orig_y.floor() as usize;
            let y1 = (y0 + 1).min(grid_height - 1);

            let wx = orig_x - x0 as f32;
            let wy = orig_y - y0 as f32;

            // Perform bilinear interpolation for u and v components
            let u00 = u_vec[y0 * grid_width + x0];
            let u01 = u_vec[y1 * grid_width + x0];
            let u10 = u_vec[y0 * grid_width + x1];
            let u11 = u_vec[y1 * grid_width + x1];

            let v00 = v_vec[y0 * grid_width + x0];
            let v01 = v_vec[y1 * grid_width + x0];
            let v10 = v_vec[y0 * grid_width + x1];
            let v11 = v_vec[y1 * grid_width + x1];

            let u = (1.0 - wx) * (1.0 - wy) * u00
                + (1.0 - wx) * wy * u01
                + wx * (1.0 - wy) * u10
                + wx * wy * u11;

            let v = (1.0 - wx) * (1.0 - wy) * v00
                + (1.0 - wx) * wy * v01
                + wx * (1.0 - wy) * v10
                + wx * wy * v11;

            new_u_vec[y * new_width + x] = u;
            new_v_vec[y * new_width + x] = v;
        }
    }

    (new_u_vec, new_v_vec)
}

fn generate(
    u_vec: &[f32],
    v_vec: &[f32],
    upscale_factor: u32,
    color_steps: &[ColorStep],
    density: f32,
    line_multiplier: Option<f32>,
    antialiasing: bool,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    if u_vec.len() != 152100 || v_vec.len() != 152100 {
        panic!("u_vec and v_vec must both have a length of 152100.");
    }

    let grid_width = 390;
    let grid_height = 390;
    let new_width = grid_width * upscale_factor as usize;
    let new_height = grid_height * upscale_factor as usize;

    let (new_u_vec, new_v_vec) =
        upscale_vectors(u_vec, v_vec, grid_width, grid_height, upscale_factor);

    let mut img = ImageBuffer::new(new_width as u32, new_height as u32);
    for pixel in img.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 0]);
    }

    // Determine the maximum speed in the grid
    let max_speed = new_u_vec
        .iter()
        .zip(new_v_vec.iter())
        .map(|(&u, &v)| (u.powi(2) + v.powi(2)).sqrt())
        .fold(0.0, f32::max);

    // Determine the step size based on the density
    let step_x = (1.0 / density.sqrt()).ceil() as usize;
    let step_y = (1.0 / density.sqrt()).ceil() as usize;

    // Determine the maximum length of the line in pixels
    let mut max_line_length_pixels = 12.5 * upscale_factor as f32;
    if let Some(line_multiplier) = line_multiplier {
        max_line_length_pixels *= line_multiplier;
    }

    // Calculate the scaling factor to map the velocity to image space
    let scale_factor = if max_speed > 0.0 {
        max_line_length_pixels / max_speed
    } else {
        0.0
    };

    for y in (0..new_height).step_by(step_y) {
        for x in (0..new_width).step_by(step_x) {
            let index = y * new_width + x;
            let u = new_u_vec[index];
            let v = new_v_vec[index];

            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;

            // Calculate magnitude of the velocity vector
            let speed = (u.powi(2) + v.powi(2)).sqrt();
            let normalized_speed = if max_speed > 0.0 {
                speed / max_speed
            } else {
                0.0
            };

            // Get color based on the speed of the velocity vector
            let color = speed_to_color(normalized_speed, 64, color_steps);

            // Scale the velocity components to fit the image size
            let scaled_u = u * scale_factor;
            let scaled_v = v * scale_factor;

            // Determine the end position of the line based on the scaled velocity
            let end_x = px + scaled_u;
            let end_y = py + scaled_v;

            // Draw the line from the start (px, py) to the calculated end (end_x, end_y)
            draw_line_with_alpha(
                &mut img,
                px as i32,
                py as i32,
                end_x as i32,
                end_y as i32,
                color,
                antialiasing,
            );
        }
    }

    img
}

fn speed_to_color(normalized_speed: f32, alpha: u8, color_steps: &[ColorStep]) -> Rgba<u8> {
    if color_steps.is_empty() {
        return Rgba([0, 0, 0, alpha]);
    }

    for i in 0..color_steps.len() - 1 {
        let current_step = &color_steps[i];
        let next_step = &color_steps[i + 1];

        if normalized_speed >= current_step.step && normalized_speed <= next_step.step {
            let t = (normalized_speed - current_step.step) / (next_step.step - current_step.step);
            let (r, g, b) = interpolate_color(current_step.color, next_step.color, t);
            return Rgba([r, g, b, alpha]);
        }
    }

    if normalized_speed < color_steps[0].step {
        Rgba([
            color_steps[0].color[0],
            color_steps[0].color[1],
            color_steps[0].color[2],
            alpha,
        ])
    } else {
        let last = color_steps.len() - 1;
        Rgba([
            color_steps[last].color[0],
            color_steps[last].color[1],
            color_steps[last].color[2],
            alpha,
        ])
    }
}

fn interpolate_color(color1: [u8; 3], color2: [u8; 3], t: f32) -> (u8, u8, u8) {
    let r = ((color1[0] as f32 * (1.0 - t) + color2[0] as f32 * t).round()) as u8;
    let g = ((color1[1] as f32 * (1.0 - t) + color2[1] as f32 * t).round()) as u8;
    let b = ((color1[2] as f32 * (1.0 - t) + color2[2] as f32 * t).round()) as u8;
    (r, g, b)
}

fn draw_line_with_alpha(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    color: Rgba<u8>,
    antialiasing: bool,
) {
    let (width, height) = img.dimensions();
    let mut dx = x1 - x0;
    let mut dy = y1 - y0;

    let sx = if dx > 0 { 1 } else { -1 };
    dx *= sx;
    let sy = if dy > 0 { 1 } else { -1 };
    dy *= sy;

    let mut err = (if dx > dy { dx } else { -dy }) / 2;

    loop {
        if x0 >= 0 && y0 >= 0 && x0 < width as i32 && y0 < height as i32 {
            if antialiasing {
                for nx in -1..=1 {
                    for ny in -1..=1 {
                        let px = (x0 + nx).clamp(0, img.width() as i32 - 1) as u32;
                        let py = (y0 + ny).clamp(0, img.height() as i32 - 1) as u32;
                        blend_pixel(img, px, py, color);
                    }
                }
            } else {
                blend_pixel(img, x0 as u32, y0 as u32, color);
            }
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = err;
        if e2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if e2 < dy {
            err += dx;
            y0 += sy;
        }
    }
}

fn blend_pixel(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: u32, y: u32, color: Rgba<u8>) {
    let src = color;
    let dst = img.get_pixel(x, y);

    let alpha_src = src[3] as f32 / 255.0;
    let alpha_dst = dst[3] as f32 / 255.0;

    let alpha_out = alpha_src + alpha_dst * (1.0 - alpha_src);
    if alpha_out == 0.0 {
        return;
    }

    let ratio_src = alpha_src / alpha_out;
    let ratio_dst = alpha_dst * (1.0 - alpha_src) / alpha_out;

    let r = (src[0] as f32 * ratio_src + dst[0] as f32 * ratio_dst) as u8;
    let g = (src[1] as f32 * ratio_src + dst[1] as f32 * ratio_dst) as u8;
    let b = (src[2] as f32 * ratio_src + dst[2] as f32 * ratio_dst) as u8;

    img.put_pixel(x, y, Rgba([r, g, b, (alpha_out * 255.0).round() as u8]));
}

#[allow(clippy::too_many_arguments)]
pub fn create_image(
    output_path: &str,
    u_vec: &[f32],                     // u component of wind in m/s
    v_vec: &[f32],                     // v component of wind in m/s
    upscale_factor: u32,               // upscale the image by a factor
    density: f32, // density of the lines (0 to 1 range) 1 is a line for every pixel
    antialiasing: bool, // whether to use antialiasing (not really antialiasing but it makes lines thicker)
    line_multiplier: Option<f32>, // line length is based on velocity and upscaling, make lines longer or shorter using this
    color_steps: Option<&[ColorStep]>, // colors and steps to interpolate between
) {
    let default_palette = get_palette(Palette::Default);
    let color_steps = color_steps.unwrap_or(&default_palette);
    let img = generate(
        u_vec,
        v_vec,
        upscale_factor,
        color_steps,
        density,
        line_multiplier,
        antialiasing,
    );
    let (width, height) = img.dimensions();
    let mut bg_img = ImageBuffer::new(width, height);

    for pixel in bg_img.pixels_mut() {
        *pixel = Rgba([30, 30, 30, 255]);
    }

    // Overlay the generated image onto the white background
    for (x, y, pixel) in img.enumerate_pixels() {
        blend_pixel(&mut bg_img, x, y, *pixel);
    }

    bg_img.save(output_path).expect("Failed to save image");
}
