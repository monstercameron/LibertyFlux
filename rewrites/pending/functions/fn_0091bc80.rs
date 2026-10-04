// original: 0x0091bc80 scaled_input_lookup
use lf_checker_rt::{export, global, relocated};

/// Scaled table value for an input code.
///
/// When disabled returns a per-mode default ratio. When mode 2 is active and
/// the code falls in one of two ranges returns a preset ratio. Otherwise maps
/// the code through a word table (with a special case for 0x3F) and scales the
/// resulting byte by a per-mode divisor.
export!(cdecl, rw_0091bc80(a0: u32) -> f32 {
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
    const DIV2: u32 = 0x1195B6C;
    const DEFDIV: u32 = 0x11956BC;
    const DEFVAL: u32 = 0x1195678;
    unsafe {
        if global::<u8>(EN).read() == 0 {
            let bl = global::<u8>(BLM).read() as u32;
            let off = bl.wrapping_mul(0x258);
            let v = (relocated(DEFVAL.wrapping_add(off)) as *const u32).read_unaligned();
            let d = (relocated(DEFDIV.wrapping_add(off)) as *const u32).read_unaligned();
            return f32::from_bits(v) / f32::from_bits(d);
        }
        let dx = (a0 & 0xFFFF) as u16;
        let bl = global::<u8>(BLM).read();
        if bl == 2 {
            let eax = dx as u32;
            let lo1 = (relocated(LO1) as *const u32).read_unaligned();
            let hi1 = (relocated(HI1) as *const u32).read_unaligned();
            if !((eax as i32) < (lo1 as i32) || (eax as i32) > (hi1 as i32)) {
                let v = (relocated(F1) as *const u32).read_unaligned();
                let d = (relocated(DIV2) as *const u32).read_unaligned();
                return f32::from_bits(v) / f32::from_bits(d);
            }
            let lo2 = (relocated(LO2) as *const u32).read_unaligned();
            let hi2 = (relocated(HI2) as *const u32).read_unaligned();
            if !((eax as i32) < (lo2 as i32) || (eax as i32) > (hi2 as i32)) {
                let v = (relocated(F2) as *const u32).read_unaligned();
                let d = (relocated(DIV2) as *const u32).read_unaligned();
                return f32::from_bits(v) / f32::from_bits(d);
            }
        }
        let bh = global::<u8>(FLAG).read();
        let mut eax: u32;
        if global::<u8>(FLG0).read() == 0 && dx == 0x3F {
            if global::<u8>(MODE).read() == 0x6A || bh != 0 {
                eax = 0xFFFF;
            } else {
                eax = 0xFD;
            }
        } else {
            let t = (bl as u32)
                .wrapping_mul(3)
                .wrapping_add(global::<u8>(BBD).read() as u32);
            let ecx = t.wrapping_shl(8).wrapping_add(dx as u32);
            let va = WORDTAB.wrapping_add(ecx.wrapping_mul(2));
            eax = (relocated(va) as *const u16).read_unaligned() as u32;
        }
        if global::<u8>(MODE).read() == 0x6A || bh != 0 {
            if (eax & 0xFFFF) as u16 == 0xFFFF {
                eax = 0xD0;
            }
        }
        let off = (bl as u32).wrapping_mul(0x258);
        let idx = (eax & 0xFFFF) as u32;
        let b = (relocated(BYTETAB.wrapping_add(off).wrapping_add(idx)) as *const u8).read();
        let d = (relocated(DEFDIV.wrapping_add(off)) as *const u32).read_unaligned();
        (b as f32) / f32::from_bits(d)
    }
});
