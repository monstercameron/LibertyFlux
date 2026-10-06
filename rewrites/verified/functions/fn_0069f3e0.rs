// original: 0x0069F3E0 stamp_kind_byte_to_targets
/// Stamp a kind-derived byte into each entry's target and sub-target.
///
/// For each `i` in `0..count` (`count` at `this+0`, loop bound signed: `jle`
/// to exit, `jl` to continue), reads the kind word at `this+8+4*i` and the
/// value word at `this+0x298+4*i` masked to 24 bits, derives a byte `cl`,
/// then writes `cl` to `tgt+6` (`tgt` at `this+0x528+4*i`) and, when the
/// sub-pointer at `tgt+0xC` is non-null, to `sub+[tgt+8]*8`.
///
/// `cl` derivation (the original is two jump tables over code-section data,
/// embedded here as constants): kinds above 15 as unsigned (`ja`) give 0;
/// kinds 1, 2, 11, 14, 15 give 0x80; kind 5 reads a global byte whose value
/// is ignored (both paths clear `cl`, a dead branch in the original);
/// kind 9 consults the 22-entry secondary table by the masked value when it
/// is at most 0x15 as unsigned (`ja`), giving 0x80 for values 0-3, 20 and 21
/// and 0 for values 4-19; every other kind gives 0. The kind and
/// masked-value comparisons are unsigned but their signedness is
/// unobservable (every out-of-range value maps to 0 either way); only the
/// loop bound is observably signed. Returns nothing meaningful (eax is a
/// leftover), so the contract compares no return value.
/// Original: thiscall, no stack words, no calls.
lf_checker_rt::export!(thiscall, rw_0069f3e0(this: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0;
        const KINDS: u32 = 8;
        const VALUES: u32 = 0x298;
        const TARGETS: u32 = 0x528;
        const VMASK: u32 = 0x00FF_FFFF;
        const SET: u8 = 0x80;
        const SUB: [u8; 22] = [
            0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
        ];
        let count = (this.wrapping_add(COUNT) as *const u32).read_unaligned();
        let mut i = 0i32;
        while i < (count as i32) {
            let u = i as u32;
            let kind = (this.wrapping_add(KINDS).wrapping_add(u.wrapping_mul(4)) as *const u32)
                .read_unaligned();
            let masked = (this.wrapping_add(VALUES).wrapping_add(u.wrapping_mul(4)) as *const u32)
                .read_unaligned()
                & VMASK;
            let mut cl = 0u8;
            if kind <= 0xF {
                match kind {
                    1 | 2 | 11 | 14 | 15 => cl = SET,
                    5 => {
                        let _ = lf_checker_rt::global::<u8>(0x018B_7A5D).read_unaligned();
                        cl = 0;
                    }
                    9 => {
                        if masked > 0x15 {
                            cl = 0;
                        } else if SUB[masked as usize] == 0 {
                            cl = SET;
                        } else {
                            cl = 0;
                        }
                    }
                    _ => cl = 0,
                }
            }
            let tgt = (this.wrapping_add(TARGETS).wrapping_add(u.wrapping_mul(4)) as *const u32)
                .read_unaligned();
            let sub = (tgt.wrapping_add(0xC) as *const u32).read_unaligned();
            (tgt.wrapping_add(6) as *mut u8).write(cl);
            if sub != 0 {
                let ix = (tgt.wrapping_add(8) as *const u8).read();
                (sub.wrapping_add((ix as u32).wrapping_mul(8)) as *mut u8).write(cl);
            }
            i += 1;
        }
        0
    }
});