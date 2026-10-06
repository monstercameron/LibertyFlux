// original: 0x00976e60 audio_kind_check_twice (proposed)

/// True when the looked-up entry has one of two kinds.
///
/// Resolves `key` through the object's vtable slot +0x14. When the 16-bit
/// kind at +0x20 of the result is 0x1E the answer is 1; otherwise the lookup
/// runs again and the answer is 1 exactly when the kind is 0x1F. Equality
/// compares. Returns the byte in AL.
/// Original: 0x00976E60 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00976e60(this: u32, key: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x14;
        const KIND: u32 = 0x20;
        const FIRST: u16 = 0x1E;
        const SECOND: u16 = 0x1F;
        let vt = (this as *const u32).read_unaligned();
        let target = ((vt.wrapping_add(SLOT)) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let r1 = f(this, key);
        if ((r1.wrapping_add(KIND)) as *const u16).read_unaligned() == FIRST {
            return 1;
        }
        let r2 = f(this, key);
        if ((r2.wrapping_add(KIND)) as *const u16).read_unaligned() == SECOND {
            1
        } else {
            0
        }
    }
});
