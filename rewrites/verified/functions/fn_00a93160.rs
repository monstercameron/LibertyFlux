// original: 0x00a93160 stream_touch_slot_then_emit (proposed)

/// Touch one set slot when live, then emit it through the sink.
///
/// The slot for the index argument is resolved like the sweep does (select
/// byte at `base[idx]`, address `set+0x00 + stride*idx`; a set select bit
/// resolves null). A nonzero slot holding a nonzero head word is touched
/// first (callee 1, cdecl/1 with the index); then the slot, or null when
/// the select bit is set, is emitted (callee 2, thiscall/1 on the set
/// object from its global).
///
/// Returns the sink's answer. A set select bit with a null slot faults on
/// the head-word read, identically on both sides. Cdecl, one argument.
lf_checker_rt::export!(cdecl, rw_00a93160(idx: u32) -> u32 {
    unsafe {
        const SET_GLOBAL: u32 = 0x012fb258;
        const BASE_OFF: u32 = 0x00;
        const SELECT_OFF: u32 = 0x04;
        const STRIDE_OFF: u32 = 0x0c;
        const SKIP_BIT: u8 = 0x80;
        let set = lf_checker_rt::global::<u32>(SET_GLOBAL).read_unaligned();
        let selbase = ((set + SELECT_OFF) as *const u32).read_unaligned();
        let slot = if (((selbase + idx) as *const u8).read() & SKIP_BIT) != 0 {
            0
        } else {
            let stride = ((set + STRIDE_OFF) as *const u32).read_unaligned();
            let base = ((set + BASE_OFF) as *const u32).read_unaligned();
            base.wrapping_add(stride.wrapping_mul(idx))
        };
        // A null slot faults on the head-word read, exactly like the
        // original; the volatile load keeps the fault from being optimised
        // into a folded branch.
        let head = if slot == 0 {
            core::ptr::read_volatile(0 as *const u32)
        } else {
            (slot as *const u32).read_unaligned()
        };
        if head != 0 {
            lf_checker_rt::callee_cdecl!(1, u32, idx);
        }
        if (((selbase + idx) as *const u8).read() & SKIP_BIT) != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, set, 0)
        } else {
            let stride = ((set + STRIDE_OFF) as *const u32).read_unaligned();
            let base = ((set + BASE_OFF) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(2, u32, set, base.wrapping_add(stride.wrapping_mul(idx)))
        }
    }
});
