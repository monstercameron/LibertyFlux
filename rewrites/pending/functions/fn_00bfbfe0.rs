// original: 0x00bfbfe0 forward_field40_copy_vec3
//! rs20f6 @0xBFBFE0: forward field +0x40 with the first arg through the shared
//! helper, then copy the vec3 at +0x34 to the destination (thiscall/2).
export!(thiscall, rw_rs20f6(this: *const u8, a1: u32, dst: *mut u32) -> u32 {
    unsafe {
        // Arg order: the original pushes the field first, so the incoming
        // arg is the callee's first parameter (stub logs top-of-stack first).
        callee_cdecl!(1, u32, a1, *(this.add(0x40) as *const u32));
        let x = *(this.add(0x34) as *const u32);
        let y = *(this.add(0x38) as *const u32);
        let z = *(this.add(0x3c) as *const u32);
        *dst = x;
        *dst.add(1) = y;
        *dst.add(2) = z;
        z
    }
});
