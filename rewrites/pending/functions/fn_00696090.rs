// original: 0x00696090 channel_clone
/// Clone a channel descriptor, optionally deep-cloning its element list.
///
/// Copies the 8-byte header, adopts the element count, allocates the
/// destination list on first use, then either clones each element through
/// the element helper (when `flag` is set) or copies the pointers.
/// Returns the element count.
export!(thiscall, rs80_696090(this: *mut u8, src: *const u8, flag: u32) -> u32 {
    unsafe {
        *((this).add(0)) = *((src).add(0));
        *((this).add(1)) = *((src).add(1));
        *((this).add(2) as *mut u16) = *((src).add(2) as *const u16);
        *((this).add(4) as *mut u16) = *((src).add(4) as *const u16);
        *((this).add(6) as *mut u16) = *((src).add(6) as *const u16);
        let count = *((src).add(0x0C) as *const u16);
        if *((this).add(0x0E) as *const u16) == 0 {
            *((this).add(0x0E) as *mut u16) = count;
            let p: u32 = if count != 0 {
                callee_stdcall!(1, u32, count as u32)
            } else {
                0
            };
            *((this).add(8) as *mut u32) = p;
        }
        *((this).add(0x0C) as *mut u16) = count;
        let dst = *((this).add(8) as *const u32) as *mut u32;
        let sarr = *((src).add(8) as *const u32) as *const u32;
        for i in 0..(count as usize) {
            let v = *sarr.add(i);
            *dst.add(i) = if (flag as u8) != 0 {
                callee_thiscall!(2, u32, v)
            } else {
                v
            };
        }
        count as u32
    }
});
