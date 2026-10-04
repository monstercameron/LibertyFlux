// original: 0x00bfc510 copy_vec3_to_34
//! rs20f19 @0xBFC510: copy the source vec3 into +0x34 (thiscall/1).

use lf_k2_rt::{export};

export!(thiscall, rw_rs20f19(this: *mut u8, src: *const u32) -> u32 {
    unsafe {
        let x = *src;
        let y = *src.add(1);
        let z = *src.add(2);
        *(this.add(0x34) as *mut u32) = x;
        *(this.add(0x38) as *mut u32) = y;
        *(this.add(0x3c) as *mut u32) = z;
        z
    }
});
