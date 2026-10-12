// original: 0x00A66C40 physics_state_reset

/// Reset the eight leading 32-bit words of a physics-state record to zero and
/// return the same record pointer in EAX. The record arrives in ECX under the
/// thiscall convention; the stores cover byte offsets zero through 28.
lf_checker_rt::export!(thiscall, rw_00a66c40(state: u32) -> u32 {
    const WORD_COUNT: usize = 8;

    unsafe {
        let words = state as *mut u32;
        for index in 0..WORD_COUNT {
            words.add(index).write_unaligned(0);
        }
    }
    state
});
