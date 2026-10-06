// original: 0x008F73A0 SubObject_Thunk

/// Tail-jump thunk (8 code bytes; the rest of the listed range is padding):
/// forward to the shared routine with `this + 0x38` and the caller's stack
/// word, returning its answer. Convention: thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_008f73a0(this: u32, a0: u32) -> u32 {
    unsafe {
        const TARGET: u32 = 1;
        const SUB_OFF: u32 = 0x38;
        lf_checker_rt::callee_thiscall!(TARGET, u32, this.wrapping_add(SUB_OFF), a0)
    }
});
