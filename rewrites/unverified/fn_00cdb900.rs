// original: 0x00CDB900 CTaskSimpleNMScriptControl::vf20

/// Flag the script-control bits whose message name matches.
///
/// Compares the message name at `a2 + 0xc` against each of the 29 tabled
/// names: on a match, sets that entry's rotating bit in the flag word at
/// `+0x30`. Empty table entries are skipped. Returns the last comparison
/// result (0 equal, 1 or -1 on the first differing unsigned byte).
///
/// Original: 0x00CDB900 (thiscall, two stack words of which the first is not
/// read, callee pops).
lf_checker_rt::export!(thiscall, rw_00cdb900(this: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const NAMES: u32 = 0x01051C48;
        const TABLE: u32 = 0x01051CC4;
        const COUNT: u32 = 29;
        const FLAGS: u32 = 0x30;
        let want = a2.wrapping_add(0x0c);
        let mut bit = 1u32;
        let mut last = 0u32;
        let mut i = 0u32;
        while i < COUNT {
            let idx = ((lf_checker_rt::relocated(NAMES) + i * 4) as *const u32).read_unaligned();
            let s = ((lf_checker_rt::relocated(TABLE) + idx.wrapping_mul(4)) as *const u32)
                .read_unaligned();
            if ((s) as *const u8).read() != 0 {
                let mut j = 0u32;
                loop {
                    let a = ((s + j) as *const u8).read();
                    let b = ((want + j) as *const u8).read();
                    if a != b {
                        last = if a < b { 0xFFFFFFFFu32 } else { 1 };
                        break;
                    }
                    if a == 0 {
                        last = 0;
                        break;
                    }
                    j += 1;
                }
                if last == 0 {
                    ((this + FLAGS) as *mut u32).write_unaligned(
                        ((this + FLAGS) as *const u32).read_unaligned() | bit);
                }
            }
            bit = bit.rotate_left(1);
            i += 1;
        }
        last
    }
});
