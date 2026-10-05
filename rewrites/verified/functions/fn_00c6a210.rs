// original: 0x00c6a210 stream_purge_no_reqbits (proposed)

/// Remove every id whose stream object has none of the required request bits set.
///
/// `slots` points to 64 entry ids. Ids that are negative are left alone;
/// every other id is looked up in the global stream-object table and the
/// entry is removed when the predicate below holds for its object, by
/// shifting every later slot down one place. The freed tail slot is set
/// to `EMPTY` (-1) and the slot is re-examined, so removal preserves the
/// order of the survivors.
///
/// Original: thiscall, no calls, reads the global object table.
lf_checker_rt::export!(thiscall, rw_00c6a210(slots: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0129_5CD8;
        const SLOTS: u32 = 64;
        const EMPTY: u32 = 0xFFFF_FFFF;
        const REQ_OFF: u32 = 0x120;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let table = lf_checker_rt::relocated(TABLE);
        let mut i = 0u32;
        while i < SLOTS {
            let id = rd32(slots.wrapping_add(i.wrapping_mul(4)));
            let mut drop = false;
            if (id as i32) >= 0 {
                let obj = rd32(table.wrapping_add(id.wrapping_mul(4)));
                drop = (rd32(obj.wrapping_add(REQ_OFF)) & 0x1E0) == 0;
            }
            if drop {
                if i < SLOTS.wrapping_sub(1) {
                    let dst = slots.wrapping_add(i.wrapping_mul(4)) as *mut u32;
                    let src = slots
                        .wrapping_add(i.wrapping_add(1).wrapping_mul(4))
                        as *const u32;
                    unsafe { core::ptr::copy(src, dst, (SLOTS - 1 - i) as usize) };
                }
                wr32(slots.wrapping_add(63u32.wrapping_mul(4)), EMPTY);
            } else {
                i = i.wrapping_add(1);
            }
        }
        0
    }
});
