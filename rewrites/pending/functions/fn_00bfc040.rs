// original: 0x00bfc040 copy_vec3_from_34
//! rs20f8 @0xBFC040: copy the vec3 at +0x34 to the destination (thiscall/1).

use lf_k2_rt::{export};

export!(thiscall, rw_rs20f8(this: *const u8, dst: *mut u32) -> u32 {
    unsafe {
        let x = *(this.add(0x34) as *const u32);
        let y = *(this.add(0x38) as *const u32);
        let z = *(this.add(0x3c) as *const u32);
        *dst = x;
        *dst.add(1) = y;
        *dst.add(2) = z;
        z
    }
});
