// original: 0x00B52BC0 euphoria_find_or_alloc_slot

/// Search three slot records spaced `0x520` bytes apart for the requested
/// signedness-neutral 32-bit identifier. Return the matching slot after
/// forwarding both frame pointers to the intercepted frame writer. If no
/// match exists, initialize the first slot whose identifier is zero, write
/// the requested identifier, and update its frames. Return three when every
/// slot is occupied. The method is thiscall with four stack arguments.
lf_checker_rt::export!(thiscall, rw_b52bc0(
    this: *mut u8,
    new_handle: u32,
    match_id: u32,
    first_frame: u32,
    second_frame: u32,
) -> u32 {
    const SLOT_STRIDE: usize = 0x520;
    const SLOT_COUNT: usize = 3;
    unsafe {
        for slot_index in 0..SLOT_COUNT {
            let slot = this.add(slot_index * SLOT_STRIDE);
            let identifier = (slot as *const u32).read_unaligned();
            if identifier == match_id {
                return lf_checker_rt::callee_thiscall!(
                    2,
                    u32,
                    slot as u32,
                    first_frame,
                    second_frame,
                );
            }
        }
        for slot_index in 0..SLOT_COUNT {
            let slot = this.add(slot_index * SLOT_STRIDE);
            let identifier = (slot as *const u32).read_unaligned();
            if identifier == 0 {
                let _ = lf_checker_rt::callee_thiscall!(1, u32, slot as u32, new_handle);
            (slot as *mut u32).write_unaligned(match_id);
                return lf_checker_rt::callee_thiscall!(
                    2,
                    u32,
                    slot as u32,
                    first_frame,
                    second_frame,
                );
            }
        }
        SLOT_COUNT as u32
    }
});
