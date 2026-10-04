// original: 0x0091c7c0 biased_input_lookup
use lf_checker_rt::{export, global, relocated};

/// Scaled table value with bias for an input code and flag.
///
/// Like the single-argument variant but adds a per-mode bias (zeroed when the
/// flag argument is clear) and applies a shared divide/add/multiply tail.
export!(cdecl, rw_0091c7c0(a0: u32, a1: u32) -> f32 {
    const EN: u32 = 0x10366BE;
    const BLM: u32 = 0x10366C1;
    const FLG0: u32 = 0x10366C0;
    const BBD: u32 = 0x10366BD;
    const MODE: u32 = 0x116C250;
    const FLAG: u32 = 0x116C253;
    const WORDTAB: u32 = 0x1193C78;
    const BYTETAB: u32 = 0x1195478;
    const F1: u32 = 0x1195B40;
    const F2: u32 = 0x1195B44;
    const LO1: u32 = 0x1195B50;
    const HI1: u32 = 0x1195B54;
    const LO2: u32 = 0x1195B58;
    const HI2: u32 = 0x1195B5C;
    const DIVS: u32 = 0x1036690;
    const ADDS: u32 = 0x10366C4;
    const MULS: u32 = 0x10366A4;
    const BIAS: u32 = 0x119567C;
    const DEFVAL: u32 = 0x1195678;
    unsafe {
        let bl0 = global::<u8>(BLM).read() as u32;
        let bd = global::<u8>(BBD).read() as u32;
        let bi = bl0.wrapping_mul(0x96).wrapping_add(bd);
        let bias_bits = (relocated(BIAS.wrapping_add(bi.wrapping_mul(4))) as *const u32).read_unaligned();
        let mut xmm1 = f32::from_bits(bias_bits);
        if (a1 & 0xFF) == 0 {
            xmm1 = 0.0;
        }
        if global::<u8>(EN).read() == 0 {
            let off = bl0.wrapping_mul(0x258);
            let v = (relocated(DEFVAL.wrapping_add(off)) as *const u32).read_unaligned();
            let mut x = f32::from_bits(v) + xmm1;
            x = x / f32::from_bits((relocated(DIVS) as *const u32).read_unaligned());
            x = x + f32::from_bits((relocated(ADDS) as *const u32).read_unaligned());
            x = x * f32::from_bits((relocated(MULS) as *const u32).read_unaligned());
            return x;
        }
        let si = (a0 & 0xFFFF) as u16;
        let bl = bl0 as u8;
        if bl == 2 {
            let eax = si as u32;
            let lo1 = (relocated(LO1) as *const u32).read_unaligned();
            let hi1 = (relocated(HI1) as *const u32).read_unaligned();
            if !((eax as i32) < (lo1 as i32) || (eax as i32) > (hi1 as i32)) {
                let v = (relocated(F1) as *const u32).read_unaligned();
                let mut x = f32::from_bits(v) / f32::from_bits((relocated(DIVS) as *const u32).read_unaligned());
                x = x + f32::from_bits((relocated(ADDS) as *const u32).read_unaligned());
                x = x * f32::from_bits((relocated(MULS) as *const u32).read_unaligned());
                return x;
            }
            let lo2 = (relocated(LO2) as *const u32).read_unaligned();
            let hi2 = (relocated(HI2) as *const u32).read_unaligned();
            if !((eax as i32) < (lo2 as i32) || (eax as i32) > (hi2 as i32)) {
                let v = (relocated(F2) as *const u32).read_unaligned();
                let mut x = f32::from_bits(v) / f32::from_bits((relocated(DIVS) as *const u32).read_unaligned());
                x = x + f32::from_bits((relocated(ADDS) as *const u32).read_unaligned());
                x = x * f32::from_bits((relocated(MULS) as *const u32).read_unaligned());
                return x;
            }
        }
        let bh = global::<u8>(MODE).read();
        let bflag = global::<u8>(FLAG).read();
        let mut eax: u32;
        if global::<u8>(FLG0).read() == 0 && si == 0x3F {
            if bh == 0x6A || bflag != 0 {
                eax = 0xFFFF;
            } else {
                eax = 0xFD;
            }
        } else {
            let t = bd.wrapping_add((bl as u32).wrapping_mul(3));
            let ecx = t.wrapping_shl(8).wrapping_add(si as u32);
            let va = WORDTAB.wrapping_add(ecx.wrapping_mul(2));
            eax = (relocated(va) as *const u16).read_unaligned() as u32;
        }
        if bh == 0x6A || bflag != 0 {
            if (eax & 0xFFFF) as u16 == 0xFFFF {
                eax = 0xD0;
            }
        }
        let off = (bl as u32).wrapping_mul(0x258);
        let idx = (eax & 0xFFFF) as u32;
        let b = (relocated(BYTETAB.wrapping_add(off).wrapping_add(idx)) as *const u8).read();
        let mut x = (b as f32) + xmm1;
        x = x / f32::from_bits((relocated(DIVS) as *const u32).read_unaligned());
        x = x + f32::from_bits((relocated(ADDS) as *const u32).read_unaligned());
        x = x * f32::from_bits((relocated(MULS) as *const u32).read_unaligned());
        x
    }
});
