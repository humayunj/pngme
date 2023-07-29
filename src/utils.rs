pub fn u8_slice_to_u32(slice: &[u8]) -> u32 {
    let mut len: u32 = 0;
    len |= (slice[0] as u32) << 24;
    len |= (slice[1] as u32) << 16;
    len |= (slice[2] as u32) << 8;
    len |= (slice[3] as u32) << 0;
    return len;
}
