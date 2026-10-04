// original: 0x0091bde0 context_scaled_lookup
use lf_checker_rt::{callee_cdecl, export, global, relocated};

/// Per-context scaled table value for an input code.
///
/// Calls the context-index provider, then behaves like the biased lookup with
/// per-context enable bytes, mode bytes, and add/multiply tail values.
export!(cdecl, rw_0091bde0(a0: u32) -> f32 {
    const MODE: u32 = 0x116C250;
    const FLAG: u32 = 0x116C253;
    const EXTRA: u32 = 0x117E6DA;
    const WORDTAB: u32 = 0x1193C78;
    const BYTETAB: u32 = 0x1195478;
    const BIAS: u32 = 0x119567C;
    const DEFVAL: u32 = 0x1195678;
    const DIVS: u32 = 0x1036690;
    const MULB: u32 = 0x119BF1C;
    const ENB: u32 = 0x119BF2E;
    const FLG30: u32 = 0x119BF30;
    const CLB: u32 = 0x119BF40;
    const EBPB: u32 = 0x119BF41;
    const ADDB: u32 = 0x119BF4C;
    const F1B: u32 = 0x1195690;
    const F2B: u32 = 0x1195694;
    const LO1B: u32 = 0x11956A0;
    const HI1B: u32 = 0x11956A4;
    const LO2B: u32 = 0x11956A8;
    const HI2B: u32 = 0x11956AC;
    unsafe {
        let ctx = callee_cdecl!(1, u32,);
        let edx = ctx.wrapping_mul(9);
        let rec = edx.wrapping_mul(8);
        let cl = (relocated(CLB.wrapping_add(rec)) as *const u8).read();
        let ebp = (relocated(EBPB.wrapping_add(rec)) as *const u8).read() as u32;
        let esi0 = cl as u32;
        let bi = esi0.wrapping_mul(0x96).wrapping_add(ebp);
        let xmm1 = f32::from_bits((relocated(BIAS.wrapping_add(bi.wrapping_mul(4))) as *const u32).read_unaligned());
        if (relocated(ENB.wrapping_add(rec)) as *const u8).read() == 0 {
            let off = esi0.wrapping_mul(0x258);
            let v = (relocated(DEFVAL.wrapping_add(off)) as *const u32).read_unaligned();
            let mut x = f32::from_bits(v) + xmm1;
            x = x / f32::from_bits((relocated(DIVS) as *const u32).read_unaligned());
            x = x + f32::from_bits((relocated(ADDB.wrapping_add(rec)) as *const u32).read_unaligned());
            x = x * f32::from_bits((relocated(MULB.wrapping_add(rec)) as *const u32).read_unaligned());
            return x;
        }
        let di = (a0 & 0xFFFF) as u16;
        if cl == 2 {
            let off = esi0.wrapping_mul(0x258);
            let ecx = di as u32;
            let lo1 = (relocated(LO1B.wrapping_add(off)) as *const u32).read_unaligned();
            let hi1 = (relocated(HI1B.wrapping_add(off)) as *const u32).read_unaligned();
            if !((ecx as i32) < (lo1 as i32) || (ecx as i32) > (hi1 as i32)) {
                let v = (relocated(F1B.wrapping_add(off)) as *const u32).read_unaligned();
                let mut x = f32::from_bits(v) / f32::from_bits((relocated(DIVS) as *const u32).read_unaligned());
                x = x + f32::from_bits((relocated(ADDB.wrapping_add(rec)) as *const u32).read_unaligned());
                x = x * f32::from_bits((relocated(MULB.wrapping_add(rec)) as *const u32).read_unaligned());
                return x;
            }
            let lo2 = (relocated(LO2B.wrapping_add(off)) as *const u32).read_unaligned();
            let hi2 = (relocated(HI2B.wrapping_add(off)) as *const u32).read_unaligned();
            if !((ecx as i32) < (lo2 as i32) || (ecx as i32) > (hi2 as i32)) {
                let v = (relocated(F2B.wrapping_add(off)) as *const u32).read_unaligned();
                let mut x = f32::from_bits(v) / f32::from_bits((relocated(DIVS) as *const u32).read_unaligned());
                x = x + f32::from_bits((relocated(ADDB.wrapping_add(rec)) as *const u32).read_unaligned());
                x = x * f32::from_bits((relocated(MULB.wrapping_add(rec)) as *const u32).read_unaligned());
                return x;
            }
        }
        let bh = global::<u8>(MODE).read();
        let bflag = global::<u8>(FLAG).read();
        let mut eax: u32;
        if (relocated(FLG30.wrapping_add(rec)) as *const u8).read() == 0
            && di == 0x3F
            && global::<u8>(EXTRA).read() == 0
        {
            if bh == 0x6A || bflag != 0 {
                eax = 0xFFFF;
            } else {
                eax = 0xFD;
            }
        } else {
            let t = esi0.wrapping_mul(3).wrapping_add(ebp);
            let ecx = t.wrapping_shl(8).wrapping_add(di as u32);
            let va = WORDTAB.wrapping_add(ecx.wrapping_mul(2));
            eax = (relocated(va) as *const u16).read_unaligned() as u32;
        }
        if bh == 0x6A || bflag != 0 {
            if (eax & 0xFFFF) as u16 == 0xFFFF {
                eax = 0xD0;
            }
        }
        let off = esi0.wrapping_mul(0x258);
        let idx = (eax & 0xFFFF) as u32;
        let b = (relocated(BYTETAB.wrapping_add(off).wrapping_add(idx)) as *const u8).read();
        let mut x = (b as f32) + xmm1;
        x = x / f32::from_bits((relocated(DIVS) as *const u32).read_unaligned());
        x = x + f32::from_bits((relocated(ADDB.wrapping_add(rec)) as *const u32).read_unaligned());
        x = x * f32::from_bits((relocated(MULB.wrapping_add(rec)) as *const u32).read_unaligned());
        x
    }
});
