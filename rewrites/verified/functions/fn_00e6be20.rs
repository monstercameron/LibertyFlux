// original: 0x00e6be20 veh_flag_table_clear_register
/// Clear 16 flag slots in a static table, then register a callback.
///
/// For `i` in `0..16`, writes a zero word at `P - 1` and a zero byte
/// at `P + 1`, where `P` starts at 0x016FC6B1 and advances by 0x30 per
/// slot. Then passes the code pointer 0x00E72C70 to the registrar
/// (stubbed, cdecl/1, caller pops the argument) and returns its answer.
/// Takes no arguments.
///
/// Original: 0x00E6BE20, cdecl, no arguments.
export!(cdecl, rw_00e6be20() -> u32 {
    unsafe {
        const P0: u32 = 0x16FC6B1;
        const SLOTS: usize = 16;
        const STRIDE: u32 = 0x30;
        const CODEPTR: u32 = 0xE72C70;
        let mut i = 0usize;
        while i < SLOTS {
            let p = relocated(P0 + i as u32 * STRIDE);
            *(p.wrapping_sub(1) as *mut u16) = 0;
            *((p + 1) as *mut u8) = 0;
            i += 1;
        }
        callee_cdecl!(1, u32, relocated(CODEPTR))
    }
});
