// original: 0x0094C450 clear_50_slot_triples (proposed)

/// Clear all 50 slot triples of an object, in index order.
///
/// `this` (ECX) is the object. Calls the slot-triple clearer (callee 1)
/// once per index `i` in `0..50`, passing `this` in ECX and `i` on the
/// stack. Ignores every callee answer; returns the last one (whatever the
/// final call left in EAX).
///
/// Original: 0x0094C450 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_0094C450(this: u32) -> u32 {
    unsafe {
        const SLOT_COUNT: u32 = 50;
        const CLEAR_TRIPLE: u32 = 1;
        let mut answer = 0u32;
        let mut i = 0u32;
        while i < SLOT_COUNT {
            answer = lf_checker_rt::callee_thiscall!(CLEAR_TRIPLE, u32, this, i);
            i += 1;
        }
        answer
    }
});
