// original: 0x00DDE260 UITextField key acceptance check
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Decide whether `key` is accepted: start from the direct classes (table
/// keys 0x20-0x22, 0x24, 0x26-0x29, 0x3E-0x3F, or ranges 0x2B-0x3C, 0x41-0x5A,
/// 0x61-0x7A, all UNSIGNED), then by the active input mode (a shared word
/// minus one selects one of five checks: mode key class 0, classes 2+3,
/// class 1, class 3, class 4) OR-ing any further acceptance in. When the
/// strict flag at `+0x20B` is clear, return the verdict; when set, an
/// accepted key must also pass the final class check, anything else returns
/// 0. Only al is written on return, so the upper 24 bits of the last helper
/// answer (or the mode word minus one when no helper ran) are preserved.
/// Original: thiscall, one stack word, result in eax.
lf_checker_rt::export!(thiscall, rw_00DDE260(this: u32, key: u32) -> u32 {
    unsafe {
        const CLASS0: u32 = 1;
        const CLASS1: u32 = 2;
        const CLASS2: u32 = 3;
        const CLASS3: u32 = 4;
        const CLASS4: u32 = 5;
        const FINAL: u32 = 6;
        const STRICT: u32 = 0x20B;
        const MODE_WORD: u32 = 0x01160CC8;
        let mut bl: u8 = ((matches!(key, 0x20..=0x22 | 0x24 | 0x26..=0x29 | 0x3E | 0x3F)
            || (key.wrapping_sub(0x2B) <= 0x11)
            || (key.wrapping_sub(0x41) <= 0x19)
            || (key.wrapping_sub(0x61) <= 0x19)) as u8);
        let mode = (lf_checker_rt::global::<u32>(MODE_WORD) as *const u32).read_unaligned();
        let mut last: u32 = mode.wrapping_sub(1);
        let idx: u32 = last;
        if idx <= 4 {
            match idx {
                0 => {
                    let a = lf_checker_rt::callee_thiscall!(CLASS0, u32, this, key);
                    last = a;
                    if (a as u8) != 0 { bl = 1; }
                }
                1 => {
                    let a = lf_checker_rt::callee_thiscall!(CLASS2, u32, this, key);
                    last = a;
                    if (a as u8) != 0 { bl = 1; }
                    let b = lf_checker_rt::callee_thiscall!(CLASS3, u32, this, key);
                    last = b;
                    if (b as u8) != 0 { bl = 1; }
                }
                2 => {
                    let a = lf_checker_rt::callee_thiscall!(CLASS1, u32, this, key);
                    last = a;
                    if (a as u8) != 0 { bl = 1; }
                }
                3 => {
                    let b = lf_checker_rt::callee_thiscall!(CLASS3, u32, this, key);
                    last = b;
                    if (b as u8) != 0 { bl = 1; }
                }
                _ => {
                    let a = lf_checker_rt::callee_thiscall!(CLASS4, u32, this, key);
                    last = a;
                    if (a as u8) != 0 { bl = 1; }
                }
            }
        }
        let strict = ((this + STRICT) as *const u8).read();
        if strict == 0 {
            (last & 0xFFFF_FF00) | (bl as u32)
        } else if bl == 0 {
            last & 0xFFFF_FF00
        } else {
            let a = lf_checker_rt::callee_thiscall!(FINAL, u32, this, key);
            if (a as u8) != 0 { (a & 0xFFFF_FF00) | 1 } else { a & 0xFFFF_FF00 }
        }
    }
});
