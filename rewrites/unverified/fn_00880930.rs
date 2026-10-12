// original: 0x00880930 audio_allocate_channel_states

/// Allocate `count * 100` bytes for channel states and initialize each state
/// when the returned allocation pointer is non-null.
///
/// The count is a signed 32-bit stack argument. The allocator is called even
/// for zero and negative counts, with the low 32 bits of `count * 100`. The
/// initialization loop runs only for a positive count, advances by 100 bytes,
/// and skips the initializer for a null current pointer while still advancing.
/// The routine returns the allocator's pointer and uses stdcall cleanup for
/// its one argument.
lf_checker_rt::export!(stdcall, rw_00880930(count: u32) -> u32 {
    unsafe {
        const STATE_SIZE_BYTES: u32 = 0x64;
        const ALLOCATE_CALLEE: u32 = 1;
        const INITIALIZE_STATE_CALLEE: u32 = 2;

        let allocation_bytes = count.wrapping_mul(STATE_SIZE_BYTES);
        let allocation = lf_checker_rt::callee_cdecl!(
            ALLOCATE_CALLEE, u32, allocation_bytes
        );
        let mut current_state = allocation;
        let mut remaining = count as i32;

        while remaining > 0 {
            if current_state != 0 {
                let _ = lf_checker_rt::callee_thiscall!(
                    INITIALIZE_STATE_CALLEE, u32, current_state
                );
            }
            current_state = current_state.wrapping_add(STATE_SIZE_BYTES);
            remaining -= 1;
        }

        allocation
    }
});
