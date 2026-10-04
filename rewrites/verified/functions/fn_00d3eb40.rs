// original: 0x00d3eb40 jump_reset_state (proposed)

/// Reset a ped's jump-related state words.
///
/// `ped` points to a ped (or task-owner) structure. The dword at `TASK_DATA`
/// (+0x224) points to a secondary block whose dword at `RESET_SLOT` (+0x288)
/// is set to `RESET_VALUE` (0x461c3c00). The flag word at `FLAGS` (+0x26c)
/// has bits 13-14 cleared (`CLEAR_MASK`); when bit 0 of the byte at
/// `AIRBORNE` (+0x118) is clear, bit 0 of `FLAGS` is set.
///
/// No return value is produced (original leaves a scratch pointer in eax).
/// No calls, no globals, no floating point.
///
/// Original: 0x00d3eb40 (stdcall, one stack word; incoming ecx unused).
lf_checker_rt::export!(stdcall, rw_00d3eb40(ped: u32) -> u32 {
    unsafe {
        const TASK_DATA: u32 = 0x224;
        const RESET_SLOT: u32 = 0x288;
        const RESET_VALUE: u32 = 0x461c_3c00;
        const FLAGS: u32 = 0x26c;
        const CLEAR_MASK: u32 = 0xffff_9fff;
        const AIRBORNE: u32 = 0x118;

        let data = ((ped + TASK_DATA) as *const u32).read_unaligned();
        ((data + RESET_SLOT) as *mut u32).write_unaligned(RESET_VALUE);
        let flags = (ped + FLAGS) as *mut u32;
        flags.write_unaligned(flags.read_unaligned() & CLEAR_MASK);
        if ((ped + AIRBORNE) as *const u8).read() & 1 == 0 {
            flags.write_unaligned(flags.read_unaligned() | 1);
        }
        0
    }
});
