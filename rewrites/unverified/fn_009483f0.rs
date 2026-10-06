// original: 0x009483F0 claim_config_slot (proposed)

/// Claim the first free 0x30-byte slot of the global table and fill it.
///
/// Scans the flag bytes at `TABLE + i * 0x30` upward from slot 0 (at most
/// 16 slots; when all are busy slot 16 past the end is used, exactly as the
/// original does). The winning slot gets flag 1 at +0, `n + index` at +4,
/// the four config dwords from `cfg` at +0x10..+0x1C (two moved as floats,
/// two as dwords, all bit-exact moves) and the float word `f` at +0x20.
/// Then callee 1 runs with (index, 0) and its answer is returned.
///
/// Original: 0x009483F0 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_009483F0(n: u32, cfg: u32, f: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x11EE2B0;
        const STRIDE: u32 = 0x30;
        const SCAN_END: u32 = 0x300;
        const REGISTER: u32 = 1;
        let base = lf_checker_rt::relocated(TABLE);
        let mut s = 0u32;
        if (base as *const u8).read() != 0 {
            let mut off = 0u32;
            loop {
                if off >= SCAN_END {
                    break;
                }
                off += STRIDE;
                s += 1;
                if (base.wrapping_add(off) as *const u8).read() == 0 {
                    break;
                }
            }
        }
        let slot = base.wrapping_add(s.wrapping_mul(STRIDE));
        (slot as *mut u8).write(1);
        (slot.wrapping_add(4) as *mut u32).write_unaligned(n.wrapping_add(s));
        (slot.wrapping_add(0x10) as *mut u32)
            .write_unaligned((cfg as *const u32).read_unaligned());
        (slot.wrapping_add(0x14) as *mut u32)
            .write_unaligned((cfg.wrapping_add(4) as *const u32).read_unaligned());
        (slot.wrapping_add(0x18) as *mut u32)
            .write_unaligned((cfg.wrapping_add(8) as *const u32).read_unaligned());
        (slot.wrapping_add(0x1C) as *mut u32)
            .write_unaligned((cfg.wrapping_add(12) as *const u32).read_unaligned());
        (slot.wrapping_add(0x20) as *mut u32).write_unaligned(f);
        lf_checker_rt::callee_cdecl!(REGISTER, u32, s, 0)
    }
});
