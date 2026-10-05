// original: 0x00C9D6C0 task_simple_ik_setup (proposed)

/// Attach the context, resolve the table-driven subtype, and fill the
/// parameter block from the globals.
///
/// `this` points to the task object and `a1` to its context. The context is
/// stored at `this+0x10` and `this+0x40`, then the signed word at `a1+0x2e`
/// indexes the global dispatch table: the selected entry's slot `0x38` is
/// invoked with the entry and the constant 7. The context's own slot `0xa0`
/// is invoked next; a nonzero answer re-invokes the same slot and follows
/// with slot `0xe0` on the result, while zero falls back to the pointer at
/// `a1+0x100`. The tail pointer's second word plus the first answer scaled
/// by `0xe0` locates the parameter block, which receives eight global floats
/// at `+0xb0`..`+0xcc`. The words at `this+0x44` (zero) and `this+0x20`,
/// `this+0x24`, `this+0x28` (1.0 each) are set, the nearby helper runs with
/// `this`, and the helper's answer with its low byte forced to 1 is returned.
///
/// Original: 0x00C9D6C0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00C9D6C0(this: u32, a1: u32) -> u32 {
    unsafe {
        const CTX_OFF: u32 = 0x40;
        const DISPATCH_TABLE: u32 = 0x1295CD8;
        const G_PARAMS: u32 = 0x1050BD0;
        const VT_SLOT_SUBTYPE: u32 = 0x38;
        const VT_SLOT_QUERY: u32 = 0xA0;
        const VT_SLOT_FOLLOW: u32 = 0xE0;
        const ONE: f32 = 1.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }

        wr32(this + 0x10, a1);
        wr32(this + CTX_OFF, a1);
        let idx = rd16(a1 + 0x2E) as i16 as i32;
        let entry = rd32(
            lf_checker_rt::relocated(DISPATCH_TABLE)
                .wrapping_add((idx as u32).wrapping_mul(4)),
        );
        let sub: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(entry) + VT_SLOT_SUBTYPE) as usize);
        let r1 = sub(entry, 7);
        let esi = rd32(this + CTX_OFF);
        let query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(esi) + VT_SLOT_QUERY) as usize);
        let tail = if query(esi) != 0 {
            let query2: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(esi) + VT_SLOT_QUERY) as usize);
            let r2b = query2(esi);
            let follow: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(r2b) + VT_SLOT_FOLLOW) as usize);
            follow(r2b)
        } else {
            rd32(esi + 0x100)
        };
        let g = lf_checker_rt::relocated(G_PARAMS);
        let p_b4 = rdf(g + 4);
        let p_b8 = rdf(g + 8);
        let p_b0 = rdf(g);
        let inner = rd32(tail + 4);
        let blk = r1
            .wrapping_mul(0xE0)
            .wrapping_add(rd32(inner));
        wrf(blk + 0xB0, p_b0);
        wrf(blk + 0xB4, p_b4);
        wrf(blk + 0xB8, p_b8);
        wrf(blk + 0xBC, rdf(g + 12));
        wrf(blk + 0xC0, rdf(g + 16));
        wrf(blk + 0xC4, rdf(g + 20));
        wrf(blk + 0xC8, rdf(g + 24));
        wrf(blk + 0xCC, rdf(g + 28));
        wr32(this + 0x44, 0);
        wrf(this + 0x20, ONE);
        wrf(this + 0x24, ONE);
        wrf(this + 0x28, ONE);
        let r5: u32 = lf_checker_rt::callee_thiscall!(5, u32, this);
        (r5 & 0xFFFF_FF00) | 1
    }
});
