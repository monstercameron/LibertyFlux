// original: 0x005d6330 table_columns_alloc_fill (proposed)
//
// Allocate the five column arrays of a table node and fill them with -1.0.
//
// `this` is a table node (the same layout `rw_005d6690` consumes): the
// signed row count at `+0xfc` sizes three float arrays stored at `+0xec`,
// `+0xf0` and `+0xf4`, and the signed count at `+0x100` sizes two more at
// `+0xe8` and `+0xf8`. Each array is requested from the game's allocator,
// reached through TLS slot 0 (`[slot]+8` is the allocator object, whose
// vtable slot at `+8` is the allocate entry), with the byte size
// `count * 4` and the constant arguments `0x10, 0`. The size multiply is
// unsigned with overflow saturating to 0xffffffff (a count at or above
// 0x40000000 unsigned, which includes every negative count). The answers
// are stored unchecked, in the order ec, f0, e8, f4, f8. Then the first
// three arrays are filled with -1.0 row by row and the last two likewise;
// the counts are SIGNED here, so a non-positive count skips its fill while
// still allocating (with a saturated size).
//
// Original: 0x005d6330 (thiscall, no stack arguments; returns the fifth
// answer when the second count is positive, the fourth answer when only
// the first is, else the fifth answer).
lf_checker_rt::export!(thiscall, rw_005d6330(this: u32) -> u32 {
    unsafe {
        const NEG_ONE_BITS: u32 = 0xbf80_0000;
        const CNT1: u32 = 0xfc;
        const CNT2: u32 = 0x100;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        /// Unsigned `count * 4`, saturating to all-ones on overflow,
        /// exactly as the original's `mul; seto; neg; or` sequence.
        #[inline(always)]
        fn alloc_size(count: u32) -> u32 {
            let (v, overflow) = count.overflowing_mul(4);
            if overflow {
                u32::MAX
            } else {
                v
            }
        }

        let tls0 = lf_checker_rt::tls_slot(0);
        let alloc = rd32(tls0.wrapping_add(8));
        let vtable = rd32(alloc);
        let malloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(8)) as usize);
        let n1u = rd32(this + CNT1);
        let n2u = rd32(this + CNT2);
        let a_ec = malloc(alloc, alloc_size(n1u), 0x10, 0);
        wr32(this + 0xec, a_ec);
        let a_f0 = malloc(rd32(tls0.wrapping_add(8)), alloc_size(rd32(this + CNT1)), 0x10, 0);
        wr32(this + 0xf0, a_f0);
        let a_e8 = malloc(rd32(tls0.wrapping_add(8)), alloc_size(rd32(this + CNT2)), 0x10, 0);
        wr32(this + 0xe8, a_e8);
        let a_f4 = malloc(rd32(tls0.wrapping_add(8)), alloc_size(rd32(this + CNT1)), 0x10, 0);
        wr32(this + 0xf4, a_f4);
        let a_f8 = malloc(rd32(tls0.wrapping_add(8)), alloc_size(rd32(this + CNT2)), 0x10, 0);
        wr32(this + 0xf8, a_f8);

        let n1 = n1u as i32;
        for k in 1..=n1 {
            let off = (k as u32).wrapping_sub(1).wrapping_mul(4);
            wr32(rd32(this + 0xec).wrapping_add(off), NEG_ONE_BITS);
            wr32(rd32(this + 0xf0).wrapping_add(off), NEG_ONE_BITS);
            wr32(rd32(this + 0xf4).wrapping_add(off), NEG_ONE_BITS);
        }
        let n2 = n2u as i32;
        for k in 1..=n2 {
            let off = (k as u32).wrapping_sub(1).wrapping_mul(4);
            wr32(rd32(this + 0xe8).wrapping_add(off), NEG_ONE_BITS);
            wr32(rd32(this + 0xf8).wrapping_add(off), NEG_ONE_BITS);
        }
        if n2 > 0 {
            a_f8
        } else if n1 > 0 {
            a_f4
        } else {
            a_f8
        }
    }
});
