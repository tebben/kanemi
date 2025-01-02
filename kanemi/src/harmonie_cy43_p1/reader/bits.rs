// Read a 32-bit IBM floating point number
pub fn read_f32_ibm(data: &[u8]) -> f32 {
    let sign = if (data[0] & 0x80) > 0 { -1.0 } else { 1.0 };
    let a = (data[0] & 0x7f) as i32;
    let b = (((data[1] as i32) << 16) + ((data[2] as i32) << 8) + data[3] as i32) as f32;

    sign * 2.0f32.powi(-24) * b * 16.0f32.powi(a - 64)
}

// Read a 16-bit signed integer in big-endian order
pub fn read_i16_be(array: &[u8]) -> i16 {
    let mut val = (array[1] as i16) + (((array[0] & 127) as i16) << 8);
    if array[0] & 0x80 > 0 {
        val = -val;
    }
    val
}

// Read a 16-bit unsigned integer in big-endian order
pub fn read_u16_be(array: &[u8]) -> u16 {
    (array[1] as u16) + ((array[0] as u16) << 8)
}

// Read a 24-bit unsigned integer in big-endian order
pub fn read_u24_be(array: &[u8]) -> u32 {
    (array[2] as u32) + ((array[1] as u32) << 8) + ((array[0] as u32) << 16)
}
