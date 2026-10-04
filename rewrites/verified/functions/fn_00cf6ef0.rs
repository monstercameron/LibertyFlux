// original: 0x00cf6ef0 climb_task_slot_reassign (proposed)

/// Reassigns the reference-counted slot at `+0x64` of a climb task: releases
/// the old occupant (when non-null) through the release callee, stores the
/// new value, retains it (when non-null) through the retain callee, and
/// returns the new value. Both callees take the old/new occupant in ecx and
/// the slot address on the stack.
///
/// Original: 0x00cf6ef0 (thiscall: ecx holds the object, one stack word).
lf_checker_rt::export!(thiscall, rw_00cf6ef0(this: u32, new_value: u32) -> u32 {
    unsafe {
        const SLOT_OFFSET: u32 = 0x64;
        const RELEASE_CALLEE: u32 = 1;
        const RETAIN_CALLEE: u32 = 2;
        let slot = this.wrapping_add(SLOT_OFFSET);
        let old = (slot as *const u32).read_unaligned();
        if old != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, old, slot);
        }
        (slot as *mut u32).write_unaligned(new_value);
        if new_value != 0 {
            lf_checker_rt::callee_thiscall!(RETAIN_CALLEE, u32, new_value, slot);
        }
        new_value
    }
});
