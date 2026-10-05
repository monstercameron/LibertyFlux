// original: 0x00AC6090 CCustomShaderEffectPedBoneDamageFX::vf8 (symbols)

/// Accumulate ranged table values into the effect's eleven parameter rows.
///
/// For each of the eleven rows and each of its three accumulator words, the
/// original scans the table at `source` (count at `+0x600`, up to `count`
/// entries with low bounds at `+ecx * 4`, high bounds `0x200` bytes on and
/// addends `0x400` bytes on) for the first entry whose unsigned range holds
/// the accumulator, and adds the addend on a hit (thiscall, one stack
/// pointer). The scan's `ecx == -1` check is dead (the index never goes
/// negative) and is not reproduced. It returns `source`.
lf_checker_rt::export!(thiscall, rw_00AC6090(this: u32, source: u32) -> u32 {
    unsafe {
        const ROWS: u32 = 0x2EC;
        const ROW_COUNT: u32 = 11;
        const ROW_STRIDE: u32 = 0x2C;
        const COUNT_OFF: u32 = 0x600;
        const HI_OFF: u32 = 0x200;
        const ADD_OFF: u32 = 0x400;
        let count = (source.wrapping_add(COUNT_OFF) as *const u32).read_unaligned() as i32;
        /// Unsigned range scan of one accumulator word; adds on a hit.
        unsafe fn accumulate(slot: u32, source: u32, count: i32) {
            unsafe {
                let cur = (slot as *const u32).read_unaligned();
                if count <= 0 {
                    return;
                }
                let mut c: i32 = 0;
                while c < count {
                    let lo = (source.wrapping_add((c as u32) * 4) as *const u32).read_unaligned();
                    if cur < lo {
                        c += 1;
                        continue;
                    }
                    let hi = (source.wrapping_add((c as u32) * 4 + HI_OFF) as *const u32)
                        .read_unaligned();
                    if cur < hi {
                        let add = (source.wrapping_add((c as u32) * 4 + ADD_OFF) as *const u32)
                            .read_unaligned();
                        (slot as *mut u32).write_unaligned(cur.wrapping_add(add));
                        return;
                    }
                    c += 1;
                }
            }
        }
        unsafe {
            let mut p = this.wrapping_add(ROWS);
            for _ in 0..ROW_COUNT {
                accumulate(p.wrapping_sub(ROW_STRIDE), source, count);
                accumulate(p, source, count);
                accumulate(p.wrapping_add(ROW_STRIDE), source, count);
                p = p.wrapping_add(4);
            }
            source
        }
    }
});
