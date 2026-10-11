// original: 0x00CBCA50 ped_blend_track_append
/// Append one weighted blend sample to a nine-entry record array. The
/// object stores a signed count at offset zero and rows span `0x20` bytes.
/// Counts at or above nine return the incoming EAX value; negative counts
/// follow the original wrapping address arithmetic. The count is re-read
/// before each store because rows can alias it. Eight stack words follow
/// the thiscall object pointer; float arguments are copied as raw bits.
lf_checker_rt::export!(thiscall, rw_00cbca50(
    this: *mut u8,
    weight: u32,
    x_bits: u32,
    y_bits: u32,
    z_bits: u32,
    field_18: u32,
    field_1c: u32,
    sample_id: u32,
    blend_bits: u32,
) -> u32 {
    const FIRST_ROW_OFFSET: isize = 4;
    const ROW_STRIDE: i32 = 0x20;
    const ROW_LIMIT: i32 = 9;

    let count = unsafe { this.cast::<i32>().read_unaligned() };
    if count >= ROW_LIMIT {
        return 0;
    }
    for (value, offset) in [
        (weight, 0x04isize), (sample_id, 0x08isize),
        (x_bits, 0x0Cisize), (y_bits, 0x10isize),
        (z_bits, 0x14isize), (field_18, 0x18isize),
        (field_1c, 0x1Cisize),
    ] {
        let count = unsafe { this.cast::<i32>().read_unaligned() };
        let row = this.wrapping_offset(
            FIRST_ROW_OFFSET + count.wrapping_mul(ROW_STRIDE) as isize,
        );
        unsafe { row.wrapping_offset(offset).cast::<u32>().write_unaligned(value) };
    }
    let count = unsafe { this.cast::<i32>().read_unaligned() };
    let row_index = count.wrapping_add(1);
    let row = this.wrapping_offset(row_index.wrapping_mul(ROW_STRIDE) as isize);
    unsafe { row.cast::<u32>().write_unaligned(blend_bits) };
    let count = unsafe { this.cast::<i32>().read_unaligned() };
    unsafe { this.cast::<i32>().write_unaligned(count.wrapping_add(1)) };
    row_index.wrapping_mul(ROW_STRIDE) as u32
});
