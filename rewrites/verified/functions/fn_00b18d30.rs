// original: 0x00b18d30 reset_four_slots (proposed)

/// Resets four consecutive 12-byte slots through the same helper.
///
/// Thiscall with no stack arguments. Calls the slot-reset helper with
/// this, this+0x0C and this+0x18, then tail-calls it with this+0x24;
/// the helper takes its slot in ECX and no stack arguments. Returns the
/// last call's result.
lf_checker_rt::export!(thiscall, rw_00b18d30(this: u32) -> u32 {
    unsafe {
        const SLOT_STRIDE: u32 = 0x0c;
        lf_checker_rt::callee_thiscall!(1, u32, this);
        lf_checker_rt::callee_thiscall!(1, u32, this.wrapping_add(SLOT_STRIDE));
        lf_checker_rt::callee_thiscall!(1, u32, this.wrapping_add(2 * SLOT_STRIDE));
        lf_checker_rt::callee_thiscall!(1, u32, this.wrapping_add(3 * SLOT_STRIDE))
    }
});
