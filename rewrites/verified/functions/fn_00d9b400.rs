// original: 0x00D9B400 CBoat::vf85

/// Return true when the vehicle's active flag is set or its stored scalar is
/// not exactly zero. The scalar comparison treats either signed zero as equal
/// and an unordered NaN as not equal. The low flag bit takes precedence.

use crate::support::{read_u8, read_u32};

lf_checker_rt::export!(thiscall, rw_00D9B400(this: u32) -> u8 {
    unsafe {
        const FLAGS: u32 = 0x1310;
        const VALUE: u32 = 0x1078;
        let active = read_u8(this.wrapping_add(FLAGS)) & 1 != 0;
        let value = f32::from_bits(read_u32(this.wrapping_add(VALUE)));
        u8::from(active || value != 0.0)
    }
});
