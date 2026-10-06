// original: 0x00962860 cursor_scan_advance
/// Scan a cursor buffer, validating and advancing, with state fallback.
///
/// Arguments are `(cursor, bound)`. The cursor holds `base` at `+0`,
/// `offset` at `+4` and a state byte at `+8`. Each iteration reads the byte
/// at `base + offset`: when non-zero the validator helper (thiscall/0,
/// cursor pointer as object) must report a non-zero low byte and the dword
/// 4 past it must be strictly below `bound` (unsigned); then the advance
/// helper (cdecl/1, the byte zero-extended) says how far `base` moves. A
/// zero byte instead steps the state to `(state + 1) % slots` (slot count
/// byte at `0x11F6FFD`), failing when the state table at `0x11F7018` holds
/// 2 there, else resetting `base` to 0 with `offset` from the pointer table
/// at `0x11F7000`. Any failure returns 1 with the deciding answer's upper
/// bits passed through; the loop otherwise continues. Cdecl, two words.
lf_checker_rt::export!(cdecl, rw_00962860(cursor: u32, bound: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0x11f6ffd;
        const ST_TAB: u32 = 0x11f7018;
        const PTR_TAB: u32 = 0x11f7000;
        const HI: u32 = 0xffff_ff00;
        loop {
            let base = (cursor as *const u32).read_unaligned();
            let off = (cursor.wrapping_add(4) as *const u32).read_unaligned();
            let at = base.wrapping_add(off);
            if (at as *const u8).read() != 0 {
                let r1: u32 = lf_checker_rt::callee_thiscall!(1, u32, at,);
                if (r1 & 0xff) == 0 {
                    return (r1 & HI) | 1;
                }
                if (at.wrapping_add(4) as *const u32).read_unaligned() >= bound {
                    return (r1 & HI) | 1;
                }
            }
            let base = (cursor as *const u32).read_unaligned();
            let off = (cursor.wrapping_add(4) as *const u32).read_unaligned();
            if ((base.wrapping_add(off)) as *const u8).read() != 0 {
                let b = (at as *const u8).read() as u32;
                let adv: u32 = lf_checker_rt::callee_cdecl!(2, u32, b);
                (cursor as *mut u32).write_unaligned(base.wrapping_add(adv));
            } else {
                let cl =
                    (cursor.wrapping_add(8) as *const u8).read() as u32;
                if (lf_checker_rt::relocated(ST_TAB).wrapping_add(cl) as *const u8).read() == 2
                {
                    return (cl & HI) | 1;
                }
                let c =
                    (lf_checker_rt::global::<u8>(SLOTS) as *const u8).read() as u32;
                let dl = (cl + 1) % c;
                (cursor as *mut u32).write_unaligned(0);
                (cursor.wrapping_add(8) as *mut u8).write(dl as u8);
                let np = (lf_checker_rt::relocated(PTR_TAB).wrapping_add(dl * 4)
                    as *const u32)
                    .read_unaligned();
                (cursor.wrapping_add(4) as *mut u32).write_unaligned(np);
            }
        }
    }
});
