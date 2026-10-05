// original: 0x00b921c0 NativeImpl_GET_BLIP_INFO_ID_POSITION

/// Writes a blip marker position into a four-word output slot.
///
/// Resolves `handle` through `LOOKUP`; id -1 zeroes the first three words.
/// Otherwise reads the marker row from `TABLE`: when its flag byte at
/// `FLAG_OFF` is set the position comes from `POS_A`, else from `POS_B`
/// with a zero third word. The fourth word is NOT a position component: the
/// original loads it from below its own frame (uninitialized scratch), so
/// the contract pins `stack_fill` to zero and this rewrite stores zero
/// (see `narrowed`). Returns the output pointer.
///
/// Original: 0x00B921C0 (cdecl, two stack words, returns the out pointer).
lf_checker_rt::export!(cdecl, rw_00b921c0(handle: u32, out: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        const TABLE: u32 = 0x0118F6F8;
        const FLAG_OFF: u32 = 8;
        const POS_A: u32 = 0x30;
        const POS_B: u32 = 0x20;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let id: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, handle);
        let o = out as *mut u32;
        if id == 0xFFFF_FFFF {
            o.write(0);
            o.add(1).write(0);
            o.add(2).write(0);
        } else {
            let row = rd32(
                lf_checker_rt::relocated(TABLE).wrapping_add(id.wrapping_mul(4)),
            );
            if (row.wrapping_add(FLAG_OFF) as *const u8).read() != 0 {
                o.write(rd32(row.wrapping_add(POS_A)));
                o.add(1).write(rd32(row.wrapping_add(POS_A + 4)));
                o.add(2).write(rd32(row.wrapping_add(POS_A + 8)));
            } else {
                o.write(rd32(row.wrapping_add(POS_B)));
                o.add(1).write(rd32(row.wrapping_add(POS_B + 4)));
                o.add(2).write(0);
            }
        }
        // Uninitialized-scratch slot: the original reads below its frame.
        o.add(3).write(0);
        out
    }
});
