// original: 0x008917a0 aud_chain_tag8_eq1
/// Walks the object chain while the tag at 0x70 (bits 8-9) reads 2, then
/// reports whether the final tag equals 1. A link byte of 0xff ends the walk
/// with 0. Only the low byte of the result is meaningful.
export!(thiscall, rw_008917a0(this: *mut u8) -> u32 {
    unsafe {
        let stride = *global::<u32>(0x115d964);
        let table = *global::<u32>(0x115d988);
        let mut node = this as u32;
        loop {
            let tag = ((*((node + 0x70) as *const u32) >> 8) & 3) as u8;
            if tag != 2 {
                return (tag == 1) as u32;
            }
            let link = *((node + 5) as *const u8);
            if link == 0xff {
                return 0;
            }
            let idx = *((node + 0x40) as *const u8) as u32;
            let entry = *((table
                .wrapping_add(idx.wrapping_mul(0x6f40))
                .wrapping_add(0x6f10)) as *const u32);
            node = stride.wrapping_mul(link as u32).wrapping_add(entry);
        }
    }
});
