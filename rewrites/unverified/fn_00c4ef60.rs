// original: 0x00C4EF60 task_pairwise_byte_xor_threshold

/// Decide whether either pair of bytes in the resolved task state differs enough.
///
/// The cdecl argument is a task record. A nonzero byte at `+0x218`, a zero
/// byte at `+0x219`, or a null state returned by the resolver yields false.
/// Otherwise, the function XORs the bytes at `+0x26ee` and `+0x26ec`, and
/// at `+0x26de` and `+0x26dc`, then returns whether the larger unsigned
/// difference is at least `0xbe`. The early false paths only define AL.
lf_checker_rt::export!(cdecl, rw_00c4ef60(record: u32) -> u32 {
    const RESOLVE_TASK_STATE: u32 = 0;
    const ENABLED_OFFSET: u32 = 0x218;
    const READY_OFFSET: u32 = 0x219;
    const STATE_HIGH_A: u32 = 0x26ee;
    const STATE_LOW_A: u32 = 0x26ec;
    const STATE_HIGH_B: u32 = 0x26de;
    const STATE_LOW_B: u32 = 0x26dc;
    const MIN_DIFFERENCE: u8 = 0xbe;

    unsafe {
        if (record.wrapping_add(ENABLED_OFFSET) as *const u8).read() != 0 {
            return 0;
        }
        if (record.wrapping_add(READY_OFFSET) as *const u8).read() == 0 {
            return 0;
        }
    }
    let state = unsafe {
        lf_checker_rt::callee_thiscall!(RESOLVE_TASK_STATE, u32, record)
    };
    if state == 0 {
        return 0;
    }

    unsafe {
        let high_a = (state.wrapping_add(STATE_HIGH_A) as *const u8).read();
        let low_a = (state.wrapping_add(STATE_LOW_A) as *const u8).read();
        let high_b = (state.wrapping_add(STATE_HIGH_B) as *const u8).read();
        let low_b = (state.wrapping_add(STATE_LOW_B) as *const u8).read();
        let difference_a = high_a ^ low_a;
        let difference_b = high_b ^ low_b;
        u32::from(difference_a.max(difference_b) >= MIN_DIFFERENCE)
    }
});
