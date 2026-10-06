// original: 0x00962750 ref_table_b_reset
/// Reset handle table B and clear the shared flag array.
///
/// Visits all `0x818` entries of the table at `0x12142D0`: a non-null entry
/// pointer has the word at `+0x64` forced to `-1`, then the entry becomes
/// pointer `0` with tag `0xFFFF`. Afterwards all `0x5DC` bytes of the flag
/// array at `0x11F6958` are cleared. The defensive tail branch for a counter
/// at or above `0x5DC` (unsigned) is unreachable by the loop bound, so the
/// rewrite traps there. Returns `0x5DC`.
lf_checker_rt::export!(cdecl, rw_00962750() -> u32 {
    unsafe {
        const TAB: u32 = 0x12142d0;
        const FLAGS: u32 = 0x11f6958;
        const ENTRIES: u32 = 0x818;
        const COUNT: u32 = 0x5dc;
        let mut i = 0u32;
        while i < ENTRIES {
            let e = lf_checker_rt::relocated(TAB).wrapping_add(i.wrapping_mul(8));
            let p = (e as *const u32).read_unaligned();
            if p != 0 {
                (p.wrapping_add(0x64) as *mut u32).write_unaligned(0xffff_ffff);
            }
            (e as *mut u32).write_unaligned(0);
            (e.wrapping_add(4) as *mut u16).write_unaligned(0xffff);
            i += 1;
        }
        let mut eax = 0u32;
        loop {
            let ecx = (eax & 0xffff) as u16 as i16 as i32;
            if (ecx as u32) >= COUNT {
                core::hint::unreachable_unchecked()
            }
            eax += 1;
            (lf_checker_rt::relocated(FLAGS).wrapping_add(ecx as u32) as *mut u8).write(0);
            if !(((eax & 0xffff) as u16 as i16) < (COUNT as u16 as i16)) {
                break;
            }
        }
        eax
    }
});
