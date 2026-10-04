// original: 0x008d21c0 NativeImpl_IS_PLAYER_IN_INFO_ZONE
/// Copies the 3-word vector at +0x30 of a resolved info object into the
/// caller's buffer. The object is the +0x20 link of the flag-selected record:
/// the +0xb30 record when the +0x26c flags have bit 2 set and that record is
/// non-null, otherwise the +0x598 record. Returns the buffer pointer.
export!(thiscall, rw_008d21c0(this_: u32, out: u32) -> u32 {
    unsafe {
        let mid = *((this_ + 0x598) as *const u32);
        let mut rec = mid;
        if *((mid + 0x26c) as *const u8) & 4 != 0 {
            let alt = *((mid + 0xb30) as *const u32);
            if alt != 0 {
                rec = alt;
            }
        }
        let src = *((rec + 0x20) as *const u32);
        let dst = out as *mut u32;
        *dst = *((src + 0x30) as *const u32);
        *dst.add(1) = *((src + 0x34) as *const u32);
        *dst.add(2) = *((src + 0x38) as *const u32);
        out
    }
});
