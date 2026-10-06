// original: 0x0094EE30 describe_if_present (proposed)

/// When the object head word is non-null, describe the object through two callees.
///
/// `this` (ECX) is the object. A null head word (`[this]`) returns at once
/// with no observable effect (EAX keeps its entry value, so no return
/// channel is compared). Otherwise callee 1 runs with the constant -1 and
/// callee 2 runs with (`this + HEAD_OFF`, callee 1's answer).
///
/// Original: 0x0094EE30 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_0094EE30(this: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x86;
        const TAG_OF: u32 = 1;
        const DESCRIBE: u32 = 2;
        let head = (this as *const u32).read_unaligned();
        if head != 0 {
            let tag = lf_checker_rt::callee_cdecl!(TAG_OF, u32, 0xFFFF_FFFF);
            lf_checker_rt::callee_cdecl!(DESCRIBE, u32, this.wrapping_add(HEAD_OFF), tag);
        }
        0
    }
});
