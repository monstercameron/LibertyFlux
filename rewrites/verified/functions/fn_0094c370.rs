// original: 0x0094C370 reset_16_subobjects (proposed)

/// Reset all 16 sub-objects of an object, in index order.
///
/// `this` (ECX) is the object. Calls the sub-object resetter (callee 1)
/// once per index `i` in `0..16`, passing `this` in ECX and `i` on the
/// stack. Ignores every callee answer; returns the last one (whatever the
/// final call left in EAX).
///
/// Original: 0x0094C370 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_0094C370(this: u32) -> u32 {
    unsafe {
        const SLOT_COUNT: u32 = 0x10;
        const RESET_SUB: u32 = 1;
        let mut answer = 0u32;
        let mut i = 0u32;
        while i < SLOT_COUNT {
            answer = lf_checker_rt::callee_thiscall!(RESET_SUB, u32, this, i);
            i += 1;
        }
        answer
    }
});
