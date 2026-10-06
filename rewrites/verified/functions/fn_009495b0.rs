// original: 0x009495B0 clear_192_slot_words (proposed)

/// Clear all 192 slot words of an object, in index order.
///
/// `this` (ECX) is the object. Calls the slot-word clearer (callee 1) once
/// per index `i` in `0..192`, passing `this` in ECX and `i` on the stack.
/// Ignores every callee answer; returns the last one (whatever the final
/// call left in EAX).
///
/// Original: 0x009495B0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_009495B0(this: u32) -> u32 {
    unsafe {
        const SLOT_COUNT: u32 = 0xC0;
        const CLEAR_WORD: u32 = 1;
        let mut answer = 0u32;
        let mut i = 0u32;
        while i < SLOT_COUNT {
            answer = lf_checker_rt::callee_thiscall!(CLEAR_WORD, u32, this, i);
            i += 1;
        }
        answer
    }
});
