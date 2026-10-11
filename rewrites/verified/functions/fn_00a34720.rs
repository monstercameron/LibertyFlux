// original: 0x00A34720 refresh_entity_world_timestamps

/// Updates the timestamp field of each active world entry when the window is
/// iconic or either global refresh flag is set. Otherwise, clears entries
/// whose unsigned elapsed time is greater than 4,000 ticks. The table has 90
/// records spaced 0x24 bytes apart; each record's active dword is at offset
/// zero and its timestamp is at +0x1c. The window-state query is an imported
/// stdcall call with the stored window handle. This no-argument cdecl routine
/// returns the relocated address immediately after the final table record in
/// EAX.
lf_checker_rt::export!(cdecl, rw_00a34720() -> u32 {
    const WINDOW_HANDLE_VA: u32 = 0x017a_ccd8;
    const CURRENT_TICK_VA: u32 = 0x0117_35b4;
    const REFRESH_FLAG_A_VA: u32 = 0x0105_b48f;
    const REFRESH_FLAG_B_VA: u32 = 0x017e_d8d1;
    const REFRESH_FLAG_C_VA: u32 = 0x0117_3590;
    const REFRESH_FLAG_D_VA: u32 = 0x0117_3591;
    const TABLE_VA: u32 = 0x012d_f0c8;
    const TABLE_END_VA: u32 = 0x012d_fd8c;
    const RECORD_STRIDE: u32 = 0x24;
    const RECORD_COUNT: u32 = 90;
    const ACTIVE_FIELD: u32 = 0;
    const TIMESTAMP_FIELD: u32 = 0x1c;
    const EXPIRY_TICKS: u32 = 4000;

    unsafe {
        let window_handle =
            (lf_checker_rt::global::<u32>(WINDOW_HANDLE_VA) as *const u32).read_unaligned();
        let iconic = lf_checker_rt::callee_stdcall!(1, u32, window_handle);
        let refresh_flag_a =
            (lf_checker_rt::global::<u8>(REFRESH_FLAG_A_VA) as *const u8).read();
        let refresh_flag_b =
            (lf_checker_rt::global::<u8>(REFRESH_FLAG_B_VA) as *const u8).read();
        let refresh_flag_c =
            (lf_checker_rt::global::<u8>(REFRESH_FLAG_C_VA) as *const u8).read();
        let refresh_flag_d =
            (lf_checker_rt::global::<u8>(REFRESH_FLAG_D_VA) as *const u8).read();
        let should_refresh = iconic != 0
            || (refresh_flag_a != 0 && refresh_flag_b != 0)
            || refresh_flag_c != 0
            || refresh_flag_d != 0;
        let current_tick =
            (lf_checker_rt::global::<u32>(CURRENT_TICK_VA) as *const u32).read_unaligned();
        let table = lf_checker_rt::global::<u8>(TABLE_VA) as u32;

        let mut index = 0u32;
        while index < RECORD_COUNT {
            let record = table.wrapping_add(index.wrapping_mul(RECORD_STRIDE));
            let active = ((record + ACTIVE_FIELD) as *const u32).read_unaligned();
            if active != 0 {
                let timestamp_slot = (record + TIMESTAMP_FIELD) as *mut u32;
                if should_refresh {
                    timestamp_slot.write_unaligned(current_tick);
                } else {
                    let timestamp = timestamp_slot.read_unaligned();
                    if current_tick > timestamp
                        && current_tick.wrapping_sub(timestamp) > EXPIRY_TICKS
                    {
                        (record as *mut u32).write_unaligned(0);
                        timestamp_slot.write_unaligned(0);
                    }
                }
            }
            index += 1;
        }

        lf_checker_rt::global::<u8>(TABLE_END_VA) as u32
    }
});
