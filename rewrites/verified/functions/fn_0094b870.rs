// original: 0x0094B870 forward_created_object (proposed)

/// Create a helper through callee 1, then tail into callee 2 with it.
///
/// `this` (ECX) is forwarded to the creator (callee 1, no stack words);
/// its answer becomes the object pointer for the tail call (callee 2, no
/// stack words), whose answer is the return value. Written as a call that
/// forwards the argument and result; the original ends in a jump.
///
/// Original: 0x0094B870 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_0094B870(this: u32) -> u32 {
    unsafe {
        const CREATE: u32 = 1;
        const CONSUME: u32 = 2;
        let created = lf_checker_rt::callee_thiscall!(CREATE, u32, this);
        lf_checker_rt::callee_thiscall!(CONSUME, u32, created)
    }
});
