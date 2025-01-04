use image::{ImageBuffer, Rgba};

pub fn generate(
    u_vec: &[f32],
    v_vec: &[f32],
    upscale_factor: u32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    if u_vec.len() != 152100 || v_vec.len() != 152100 {
        panic!("u_vec and v_vec must both have a length of 152100.");
    }

    let grid_width = 390;
    let grid_height = 390;

    let width = (grid_width as u32) * upscale_factor;
    let height = (grid_height as u32) * upscale_factor;

    let mut img = ImageBuffer::new(width, height);

    // Fill the image with a transparent background
    for pixel in img.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 0]);
    }

    // Find the maximum wind speed for normalization
    let max_speed = u_vec
        .iter()
        .zip(v_vec.iter())
        .map(|(&u, &v)| (u.powi(2) + v.powi(2)).sqrt())
        .fold(0.0, f32::max);

    for y in 0..grid_height {
        for x in 0..grid_width {
            let index = y * grid_width + x;
            let u = u_vec[index];
            let v = v_vec[index];

            // Calculate the position of the particle in the upscaled image
            let px = (x as f32 * upscale_factor as f32) + upscale_factor as f32 / 2.0;
            let py = (y as f32 * upscale_factor as f32) + upscale_factor as f32 / 2.0;

            // Calculate the speed
            let speed = (u.powi(2) + v.powi(2)).sqrt();
            let normalized_speed = if max_speed > 0.0 {
                speed / max_speed
            } else {
                0.0
            };
            let color = speed_to_color(normalized_speed, 64); // Set alpha to 128 for 50% opacity

            draw_line_with_alpha(
                &mut img,
                px as i32,
                py as i32,
                (px + u * upscale_factor as f32) as i32,
                (py + v * upscale_factor as f32) as i32,
                color,
            );
        }
    }

    img
}

fn speed_to_color(normalized_speed: f32, alpha: u8) -> Rgba<u8> {
    // Define the base colors for low and high speeds
    let low_speed_color = [0, 150, 255]; // Bright blue
    let mid_speed_color = [255, 165, 0]; // Orange
    let high_speed_color = [255, 0, 0]; // Red

    // Calculate interpolated color based on normalized speed
    let (r, g, b) = if normalized_speed < 0.5 {
        // Interpolate between low and mid colors for speeds below 0.5
        let t = normalized_speed * 2.0; // Scale to [0, 1]
        interpolate_color(low_speed_color, mid_speed_color, t)
    } else {
        // Interpolate between mid and high colors for speeds above 0.5
        let t = (normalized_speed - 0.5) * 2.0; // Scale to [0, 1]
        interpolate_color(mid_speed_color, high_speed_color, t)
    };

    Rgba([r, g, b, alpha])
}

fn interpolate_color(color1: [u8; 3], color2: [u8; 3], t: f32) -> (u8, u8, u8) {
    let r = ((color1[0] as f32 * (1.0 - t) + color2[0] as f32 * t).round()) as u8;
    let g = ((color1[1] as f32 * (1.0 - t) + color2[1] as f32 * t).round()) as u8;
    let b = ((color1[2] as f32 * (1.0 - t) + color2[2] as f32 * t).round()) as u8;
    (r, g, b)
}

// fn speed_to_color(normalized_speed: f32, alpha: u8) -> Rgba<u8> {
//     // Map normalized speed to a color gradient from blue to red
//     let r = (normalized_speed * 255.0).min(255.0) as u8;
//     let g = 0;
//     let b = ((1.0 - normalized_speed) * 255.0).min(255.0) as u8;
//     Rgba([r, g, b, alpha])
// }

// fn speed_to_color2(normalized_speed: f32, alpha: u8) -> Rgba<u8> {
//     let r = (normalized_speed * 255.0).min(255.0) as u8;
//     let g = ((normalized_speed * 16.0).min(16.0)) as u8; // Add a small amount of green for brightness
//     let b = ((1.0 - normalized_speed) * 192.0 + 63.0).min(255.0) as u8; // Increase base blue value
//     Rgba([r, g, b, alpha])
// }

fn draw_line_with_alpha(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    color: Rgba<u8>,
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
            blend_pixel(img, x0 as u32, y0 as u32, color);
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

pub fn create_image(u_vec: &[f32], v_vec: &[f32]) {
    let upscale_factor = 3;
    let img = generate(u_vec, v_vec, upscale_factor);

    //img.save("flow_field2.png").expect("Failed to save image");

    let (width, height) = img.dimensions();
    let mut bg_img = ImageBuffer::new(width, height);
    for pixel in bg_img.pixels_mut() {
        *pixel = Rgba([30, 30, 30, 255]);
    }

    // Overlay the generated image onto the white background
    for (x, y, pixel) in img.enumerate_pixels() {
        blend_pixel(&mut bg_img, x, y, *pixel);
    }

    // Save the resulting image
    bg_img.save("flow_field.png").expect("Failed to save image");
}
