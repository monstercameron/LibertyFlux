// original: 0x0091bfd0 string_width_accumulator
use lf_checker_rt::{callee_cdecl, export, global};

/// Accumulate scaled input values over a wide string.
///
/// Resolves a gate call, optionally seeds the accumulator from the scaled
/// lookup, then adds one scaled lookup per non-zero wide character. The
/// per-index table scaling in the original nets to zero (scaled then
/// restored) and is omitted. Returns the index times nine, or zero when gated.
export!(cdecl, rw_0091bfd0(wstr: u32, fptr: u32, idx: u32) -> u32 {
    const GATE: u32 = 0x11609F6;
    const MODE: u32 = 0x116C250;
    const FLAG: u32 = 0x116C253;
    unsafe {
        let r = callee_cdecl!(1, u32, wstr);
        let first = (wstr as *const u16).read_unaligned();
        let skip = global::<u8>(GATE).read() != 0 || (r == 1 && first == 0x7E);
        if !skip {
            let use_full = global::<u8>(MODE).read() == 0x6A || global::<u8>(FLAG).read() != 0;
            let arg = if use_full { 0xFFFFu32 } else { 0xFDu32 };
            let v: f32 = callee_cdecl!(2, f32, arg);
            let mut x = v * 2.0f32;
            if use_full {
                x = x * f32::from_bits(0x3ECCCCCDu32);
            }
            let acc = fptr as *mut f32;
            acc.write(acc.read_unaligned() + x);
        }
        let mut p = wstr;
        loop {
            let c = (p as *const u16).read_unaligned();
            if c == 0 {
                break;
            }
            let v: f32 = callee_cdecl!(2, f32, (c as u32).wrapping_sub(0x20));
            let acc = fptr as *mut f32;
            acc.write(acc.read_unaligned() + v);
            p = p.wrapping_add(2);
        }
        if skip {
            0
        } else {
            idx.wrapping_mul(9)
        }
    }
});
