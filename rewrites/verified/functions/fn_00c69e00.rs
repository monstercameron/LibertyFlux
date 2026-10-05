// original: 0x00C69E00 stream_remove_id (proposed)

/// Remove one id from a 64-slot streaming id array.
///
/// `slots` points to 64 little-endian entry ids. The first slot holding
/// `wanted` is removed by shifting every later slot down one place; the
/// last slot is then set to `EMPTY` (-1). A match in the last slot only
/// writes the sentinel. If no slot holds `wanted` the array is untouched.
/// Entry order is otherwise preserved.
///
/// Original: thiscall with one stack word, no calls, no globals.
lf_checker_rt::export!(thiscall, rw_00c69e00(slots: u32, wanted: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 64;
        const EMPTY: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let mut i = 0u32;
        while i < SLOTS {
            if rd32(slots.wrapping_add(i.wrapping_mul(4))) == wanted {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i < SLOTS {
            if i < SLOTS.wrapping_sub(1) {
                let dst = slots.wrapping_add(i.wrapping_mul(4)) as *mut u32;
                let src =
                    slots.wrapping_add(i.wrapping_add(1).wrapping_mul(4)) as *const u32;
                unsafe { core::ptr::copy(src, dst, (SLOTS - 1 - i) as usize) };
            }
            wr32(slots.wrapping_add(63u32.wrapping_mul(4)), EMPTY);
        }
        0
    }
});
