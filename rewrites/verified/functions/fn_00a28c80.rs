// original: 0x00a28c80 camera_profile_adjust_scalar_from_tables

/// Adjust the camera scalar from the selected pair of 33-entry profile tables. If flag bit zero at offset 0x200 is set, subtract the first table entry from 45 degrees; then subtract the second entry times the multiplier, subtract the offset, halve the result, add the offset, and store at 0x60.
/// The original indexes both tables without checking the index; this proof covers only valid indices 0 through 32.
lf_checker_rt::export!(thiscall, rw_00a28c80(this: u32, index: u32, multiplier_bits: u32, offset_bits: u32) -> u32 {
    const PROFILE_FLAG: u32 = 0x200;
    const OUTPUT: u32 = 0x60;
    const PROFILE_BASE: f32 = f32::from_bits(0x4234_0000);
    const PROFILE_FACTOR: f32 = f32::from_bits(0x3f00_0000);
    const TABLE_A_BITS: [u32; 33] = [
        0x41c80000, 0x00000000, 0x41200000, 0x00000000, 0x41700000, 0x41c80000,
        0x41f00000, 0x41200000, 0x00000000, 0x41200000, 0x00000000, 0x41200000,
        0x41200000, 0x41200000, 0x41200000, 0x41200000, 0x41200000, 0x41200000,
        0x41200000, 0x00000000, 0x41200000, 0x00000000, 0x41f00000, 0x41f00000,
        0x41f00000, 0x41f00000, 0x41f00000, 0x41f00000, 0x41f00000, 0x41f00000,
        0x41f00000, 0x41f00000, 0x00000000,
    ];
    const TABLE_B_BITS: [u32; 33] = [
        0x00000000, 0x00000000, 0x41200000, 0x00000000, 0x41200000, 0x41200000,
        0x41200000, 0x41200000, 0x41200000, 0x41200000, 0x00000000, 0x3f2a7efa,
        0x3eaa7efa, 0x3e6353f8, 0x3f000000, 0x3f000000, 0xbfc3d70a, 0x3fc3d70a,
        0xbfaf5c29, 0x3feccccd, 0xbfbc28f6, 0x3fbc28f6, 0xbfeccccd, 0x3fc8f5c3,
        0xbfbc28f6, 0x3fbc28f6, 0xbfc8f5c3, 0x3feccccd, 0x3f000000, 0x3f000000,
        0x3f000000, 0x3f800000, 0x3f800000,
    ];
    unsafe {
        let selected_index = index as usize;
        let table_a = f32::from_bits(TABLE_A_BITS[selected_index]);
        let table_b = f32::from_bits(TABLE_B_BITS[selected_index]);
        let has_profile_flag = ((this.wrapping_add(PROFILE_FLAG) as *const u8).read() & 1) != 0;
        let base = if has_profile_flag { core::hint::black_box(PROFILE_BASE) - core::hint::black_box(table_a) } else { PROFILE_BASE };
        let multiplier = f32::from_bits(multiplier_bits);
        let offset = f32::from_bits(offset_bits);
        let product = core::hint::black_box(table_b) * core::hint::black_box(multiplier);
        let after_table = core::hint::black_box(base) - core::hint::black_box(product);
        let delta = core::hint::black_box(after_table) - core::hint::black_box(offset);
        let scaled_delta = core::hint::black_box(delta) * core::hint::black_box(PROFILE_FACTOR);
        let result = core::hint::black_box(scaled_delta) + core::hint::black_box(offset);
        (this.wrapping_add(OUTPUT) as *mut f32).write_unaligned(result);
    }
    0
});
