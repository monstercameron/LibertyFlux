// original: 0x00bfc060 expand_int8vec3_scaled
//! rs20f9 @0xBFC060: expand the signed-byte vec3 at +0x20 to floats, scale by
//! the shared factor and store to the destination (thiscall/1). Returns the
//! last byte sign-extended, like the original's leftover EAX.

use lf_k2_rt::{export, global};

export!(thiscall, rw_rs20f9(this: *const u8, dst: *mut u32) -> u32 {
    unsafe {
        let scale = *global::<f32>(0xFE8700);
        let mut last = 0i32;
        for i in 0..3usize {
            let b = *this.add(0x20 + i) as i8 as i32;
            last = b;
            *dst.add(i) = (b as f32 * scale).to_bits();
        }
        last as u32
    }
});
