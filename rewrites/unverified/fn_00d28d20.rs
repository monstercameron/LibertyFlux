// original: 0x00d28d20 CPedTargetting::vf12 (symbols)

/// Return the shared estimator's confidence: 1.0 when unset, else 0.75.
///
/// Resolves the target chain (`this + 0x24c`, `+0x224`, `+0xd8`) to a handle.
/// Lazily creates the shared estimator the first time (allocating a block
/// with the heap helper and constructing into it with the setup helper, both
/// intercepted, then publishing it to the global slot; a failed allocation
/// publishes null) and queries it with the handle (intercepted). Returns 1.0
/// when the answer's bits 13..14 are zero, else 0.75, in ST0. The two stack
/// arguments are unused.
///
/// Original: 0x00D28D20 (thiscall, two stack arguments).
lf_checker_rt::export!(thiscall, rw_00d28d20(this: u32, _a0: u32, _a1: u32) -> f32 {
    unsafe {
        const TARGET_OFF: u32 = 0x24c;
        const DATA_OFF: u32 = 0x224;
        const HANDLE_OFF: u32 = 0xd8;
        const ESTIMATOR_SLOT: u32 = 0x0167_e3b4;
        const BLOCK_FLAGS: u32 = 0x20020;
        const ANSWER_OFF: u32 = 0x8f4;
        const CONF_SHIFT: u32 = 13;
        const CONF_MASK: u32 = 3;
        const SET_BITS: u32 = 0x3f40_0000; // 0.75f
        let target = unsafe { ((this + TARGET_OFF) as *const u32).read_unaligned() };
        let data = unsafe { ((target + DATA_OFF) as *const u32).read_unaligned() };
        let handle = unsafe { ((data + HANDLE_OFF) as *const u32).read_unaligned() };
        let slot = lf_checker_rt::relocated(ESTIMATOR_SLOT);
        let mut est = unsafe { (slot as *const u32).read_unaligned() };
        if est == 0 {
            let block: u32 = lf_checker_rt::callee_cdecl!(1, u32, BLOCK_FLAGS);
            est = if block == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(2, u32, block)
            };
            unsafe { (slot as *mut u32).write_unaligned(est) };
        }
        let answer: u32 = lf_checker_rt::callee_thiscall!(3, u32, est, handle);
        let conf = (unsafe { ((answer + ANSWER_OFF) as *const u32).read_unaligned() } >> CONF_SHIFT) & CONF_MASK;
        if conf < 1 {
            1.0
        } else {
            f32::from_bits(SET_BITS)
        }
    }
});
