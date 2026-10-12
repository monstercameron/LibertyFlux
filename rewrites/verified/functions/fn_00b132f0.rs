// original: 0x00B132F0 detach_timing_slot_27c

/// Release the optional timing-node reference stored in this object's slot.
///
/// `this` points to the containing object. The slot is the 32-bit pointer at
/// offset `0x27c`. If it is non-null, the helper receives the pointed-to
/// node in ECX and the address of the slot as its sole stack argument. The
/// slot is then cleared on both paths. The original has no defined return.
lf_checker_rt::export!(thiscall, rw_00B132F0(this: u32) -> u32 {
    unsafe {
        const SLOT_OFFSET: u32 = 0x27c;
        let slot = this.wrapping_add(SLOT_OFFSET);
        let node = (slot as *const u32).read_unaligned();
        if node != 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, node, slot);
        }
        (slot as *mut u32).write_unaligned(0);
    }
    0
});
