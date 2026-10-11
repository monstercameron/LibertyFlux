// original: 0x00AE91A0 capture_frame_timing_values

/// Invokes the timing-source callback with the manager, copies four raw
/// single-precision values from manager offsets `+0x70`, `+0x74`, `+0x78`
/// and `+0x7c` into four global slots, then resets two timing flags. The
/// float fields are copied as bits, preserving every NaN payload.
///
/// Calling convention: cdecl with one pointer to the manager's fixed 32-bit
/// layout. The callback receives that pointer as its single stack argument.
lf_checker_rt::export!(cdecl, rw_00ae91a0(manager: u32) -> () {
    unsafe {
        const MANAGER_FLOATS: [u32; 4] = [0x70, 0x74, 0x78, 0x7c];
        const FLOAT_GLOBALS: [u32; 4] = [0x015c_3bc0, 0x015c_3bc4, 0x015c_3bc8, 0x015c_3bcc];
        const CALLBACK_OBJECT_VA: u32 = 0x015b_2d00;
        const TIMING_CALLBACK: u32 = 1;
        const FRAME_COUNTER_VA: u32 = 0x0159_b764;
        const TIMING_MODE_VA: u32 = 0x0103_f6d0;

        lf_checker_rt::callee_thiscall!(
            TIMING_CALLBACK,
            u32,
            lf_checker_rt::relocated(CALLBACK_OBJECT_VA),
            manager,
        );

        let mut index = 0usize;
        while index < MANAGER_FLOATS.len() {
            let bits = (manager.wrapping_add(MANAGER_FLOATS[index]) as *const u32)
                .read_unaligned();
            lf_checker_rt::global::<u32>(FLOAT_GLOBALS[index]).write_unaligned(bits);
            index += 1;
        }

        lf_checker_rt::global::<u32>(FRAME_COUNTER_VA).write_unaligned(0);
        lf_checker_rt::global::<u32>(TIMING_MODE_VA).write_unaligned(u32::MAX);
    }
});
