// original: 0x00962690 object_table_teardown
/// Tear down the 0x5DC-entry object table at `0x11F7110`.
///
/// Each 8-byte entry holds a pointer and a flag byte. Entries whose flag is
/// set resolve one indirection further: when the doubly-resolved target is
/// non-null its words at `+0xA4`/`+0xA8`/`+0xAC` reset to (`-1`, `0`, `0`);
/// entries with a clear flag reset the singly-resolved target the same way
/// whenever the pointer is non-null. Every entry is then cleared, as is its
/// byte in the flag array at `0x11F6958`. The defensive branch for an index
/// at or above `0x5DC` (unsigned) is unreachable: the signed 16-bit counter
/// never reaches the bound, so the rewrite traps there. Returns `0x5DC - 1`.
lf_checker_rt::export!(cdecl, rw_00962690() -> u32 {
    unsafe {
        const TAB: u32 = 0x11f7110;
        const FLAGS: u32 = 0x11f6958;
        const COUNT: u32 = 0x5dc;
        let mut dx = 0u32;
        let mut eax = 0u32;
        loop {
            eax = (dx & 0xffff) as u16 as i16 as i32 as u32;
            if eax >= COUNT {
                core::hint::unreachable_unchecked()
            }
            let base = lf_checker_rt::relocated(TAB).wrapping_add(eax.wrapping_mul(8));
            let bl = (base.wrapping_add(4) as *const u8).read();
            let ptr = (base as *const u32).read_unaligned();
            let mut target = 0u32;
            if bl != 0 {
                if ptr != 0 {
                    let q = (ptr as *const u32).read_unaligned();
                    if q != 0 {
                        target = q;
                    }
                }
            } else if ptr != 0 {
                target = ptr;
            }
            if target != 0 {
                (target.wrapping_add(0xac) as *mut u32).write_unaligned(0);
                (target.wrapping_add(0xa8) as *mut u32).write_unaligned(0);
                (target.wrapping_add(0xa4) as *mut u32).write_unaligned(0xffff_ffff);
            }
            (base as *mut u32).write_unaligned(0);
            (base.wrapping_add(4) as *mut u8).write(0);
            (lf_checker_rt::relocated(FLAGS).wrapping_add(eax) as *mut u8).write(0);
            dx += 1;
            if !(((dx & 0xffff) as u16 as i16) < (COUNT as u16 as i16)) {
                break;
            }
        }
        eax
    }
});
