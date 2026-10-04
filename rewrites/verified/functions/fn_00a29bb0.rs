// original: 0x00a29bb0 task_reset_release (proposed)

/// Reset a ped task object: install this class's function table, release the
/// owned link object, clear two status slots through a sub-object, and finish
/// with the base-class routine.
///
/// `this` points to the task. The double-word at `+LINK_OFF` is an owned
/// pointer: when non-null the release callee (id 1) runs on it, the slot is
/// re-read, and when still non-null the destroy callee (id 2) and the freeing
/// callee (id 3, cdecl) run on it before the slot is cleared. The
/// double-word at `+SUB_OFF` is either null or points 0x70 bytes below the
/// sub-object base; each of the slots at base `+SLOT_A` / `+SLOT_B`, when
/// non-zero, is resolved through the id 4/5/6 chain (reader global, lookup,
/// use) and then cleared. Slot `+SLOT_C` of the sub-object is always written
/// through the id 7 callee (thiscall, one stack word, always zero). Control
/// then passes to the base routine (id 8) with `this` still in ECX; the
/// original reaches it with a tail jump, the rewrite with a call that
/// returns its value.
///
/// Edge cases: a null link skips the release block; a null sub-object
/// faults reading near address zero on both sides (fault parity); a zero
/// slot skips its resolve-and-clear block. The id 4 callee ignores its ECX
/// (it reloads ECX from its stack argument), so it is declared stdcall.
///
/// Original: 0x00a29bb0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00a29bb0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00e9_bbcc;
        const LINK_OFF: u32 = 0x0e98;
        const SUB_OFF: u32 = 0x0228;
        const SUB_BIAS: u32 = 0x0070;
        const SLOT_A: u32 = 0x03e4;
        const SLOT_B: u32 = 0x03e8;
        const RESOLVER_GLOBAL: u32 = 0x016d_d63c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn sub_base(this: u32) -> u32 {
            unsafe {
                let p = rd32(this.wrapping_add(SUB_OFF));
                if p != 0 { p.wrapping_add(SUB_BIAS) } else { 0 }
            }
        }
        #[inline(always)]
        unsafe fn clear_slot(this: u32, slot: u32) {
            unsafe {
                let base = sub_base(this);
                if rd32(base.wrapping_add(slot)) != 0 {
                    let v = rd32(base.wrapping_add(slot));
                    let _reader =
                        rd32(lf_checker_rt::relocated(RESOLVER_GLOBAL));
                    let r1: u32 = lf_checker_rt::callee_stdcall!(4, u32, v);
                    let r2: u32 = lf_checker_rt::callee_cdecl!(5, u32, r1);
                    let _r3: u32 = lf_checker_rt::callee_cdecl!(6, u32, r2);
                    wr32(sub_base(this).wrapping_add(slot), 0);
                }
            }
        }

        let first = rd32(this.wrapping_add(LINK_OFF));
        wr32(this, lf_checker_rt::relocated(VTABLE));
        if first != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, first);
            let again = rd32(this.wrapping_add(LINK_OFF));
            if again != 0 {
                lf_checker_rt::callee_thiscall!(2, u32, again);
                lf_checker_rt::callee_cdecl!(3, u32, again);
            }
            wr32(this.wrapping_add(LINK_OFF), 0);
        }
        clear_slot(this, SLOT_A);
        clear_slot(this, SLOT_B);
        lf_checker_rt::callee_thiscall!(7, u32, sub_base(this), 0);
        lf_checker_rt::callee_thiscall!(8, u32, this)
    }
});
