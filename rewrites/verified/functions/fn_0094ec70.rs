// original: 0x0094EC70 release_flagged_entries (proposed)

/// Release the live entries of a 25-entry table through three callees.
///
/// Each 12-byte entry at `TABLE` holds a handle at +0, scratch at +4 and
/// an index at +8. A null handle skips the entry. Otherwise callee 1 runs
/// with (handle, 0), callee 2 as a thiscall with the handle in ECX and the
/// index on the stack, and the probe object at `INDEX` + index * 4 is read:
/// when its word at +0x52 is negative (SIGNED compare) bit 0x8000000 of the
/// handle's word at +0x28 is cleared, else set. Then byte 2 is written at
/// handle + 0x41, callee 3 runs with (handle, 0), and the entry is cleared
/// (handle 0, the other two words -1). No return channel.
///
/// Original: 0x0094EC70 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_0094EC70() -> u32 {
    unsafe {
        const TABLE: u32 = 0x11D9110;
        const COUNT: u32 = 25;
        const STRIDE: u32 = 0xC;
        const INDEX: u32 = 0x1295CD8;
        const PREP: u32 = 1;
        const SYNC: u32 = 2;
        const EMIT: u32 = 3;
        const EMPTY: u32 = 0xFFFF_FFFF;
        const BIT: u32 = 0x0800_0000;
        let mut i = 0u32;
        while i < COUNT {
            let e = lf_checker_rt::relocated(TABLE).wrapping_add(i.wrapping_mul(STRIDE));
            let h = (e as *const u32).read();
            if h != 0 {
                lf_checker_rt::callee_cdecl!(PREP, u32, h, 0);
                let k = (e.wrapping_add(8) as *const u32).read();
                lf_checker_rt::callee_thiscall!(SYNC, u32, h, k);
                let k2 = (e.wrapping_add(8) as *const u32).read();
                let probe = (lf_checker_rt::relocated(INDEX).wrapping_add(k2.wrapping_mul(4))
                    as *const u32)
                    .read();
                let w = (probe.wrapping_add(0x52) as *const u16).read_unaligned();
                let hh = (e as *const u32).read();
                let cur = (hh.wrapping_add(0x28) as *const u32).read_unaligned();
                if (w as i16) < 0 {
                    (hh.wrapping_add(0x28) as *mut u32).write_unaligned(cur & !BIT);
                } else {
                    (hh.wrapping_add(0x28) as *mut u32).write_unaligned(cur | BIT);
                }
                let hhh = (e as *const u32).read();
                (hhh.wrapping_add(0x41) as *mut u8).write(2);
                lf_checker_rt::callee_cdecl!(EMIT, u32, hhh, 0);
                (e as *mut u32).write(0);
                (e.wrapping_add(4) as *mut u32).write(EMPTY);
                (e.wrapping_add(8) as *mut u32).write(EMPTY);
            }
            i += 1;
        }
        0
    }
});
