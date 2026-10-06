// original: 0x0059EFC0 memsize_estimate (proposed)
use lf_checker_rt::{callee_cdecl, callee_stdcall, export, global, relocated};

/// Estimate a memory footprint value and store it through the out-pointer.
///
/// `out` (ECX) receives one float; the u32 return is an integer total. The
/// estimate mixes a filler callee's two scale words, a gated multiplier, six
/// polled helpers, several global knobs, the machine's total RAM, one table
/// lookup and a 64-bit counter:
///
/// - The filler callee writes two scale words (`cx`, `dx`) through the first
///   two of four scratch pointers (the other two are never read back); the
///   base is `36*cx*dx`, plus `16*G*cx*dx` when the factor global `GFACTOR`
///   is nonzero (all wrapping).
/// - A gate pair selects a multiplier: 2 when the first gate's LOW BYTE is
///   nonzero and the second gate's answer is at most 1 UNSIGNED, else 1.
/// - Three helper pairs are polled (each answer feeds the next call); the
///   accumulator is `mult*base + B + D + F`, plus a shift-knob term:
///   `(0x80 << (cl & 31))^2 * 20` where `cl` is the low byte of `SHIFTKNOB`
///   (x86 masks the count to 5 bits).
/// - A decrement knob `DECKNOB`: `d = DECKNOB - 1` (wrapping); when `d` is
///   negative as i32 (i.e. `DECKNOB == 0` or above `0x80000000`) the square
///   term uses `0x10`, else `(0x80 << (d & 31))^2`, and the shift count below
///   uses 0 instead of `d`. The accumulator gains
///   `12 * (3*(sqA + sqD) + 0x200000)` where `sqD = (0x80 << ((d'+1) & 31))^2`.
/// - A float factor `F1 = ((F1INT as i32 as f32) * C1) * C2 + C3`
///   (SIGNED int conversion, pinned order).
/// - RAM sizing like the sibling setters (cached pair, GlobalMemoryStatusEx
///   refresh, UNSIGNED bound): small memory leaves the index at 0, otherwise
///   (and on query failure) the index is `OUT_A` (the sibling's output).
/// - `E = (TABLE[index] << 20) + ADD0 + ADD1` (wrapping); `T = (E as f32`;
///   the original reaches it through a signed-double conversion with a
///   sign-dependent `+0/+2^32` correction, which equals the direct unsigned
///   conversion exactly). The integer total's low part is `trunc(T * F1)`
///   with x86 truncate semantics (NaN, infinities and out-of-range yield
///   `0x80000000`).
/// - The one-time init flag runs the same two setup calls as the sibling.
/// - A stamp callee (float argument passed on the x87 stack, unlogged by the
///   checker; its value is `(f32)total_lo`, covered through the integer)
///   answers a u64; `DIFF = COUNTER64 - answer - accum` (wrapping 64-bit).
/// - The stored float is `1 - (((DIFF as i64 as f32) * 2) / (total_lo as
///   i32 as f32))` (SIGNED conversions, pinned order; a zero divisor yields
///   IEEE infinities/NaN deterministically). The return is
///   `total_lo + accum` (wrapping).
///
/// The original leaves one value on the x87 stack (the pre-call `fld` has no
/// matching pop on the stubbed path); the x87-state check is off, so that
/// depth difference is unobserved -- the pushed value itself is covered
/// through the integer total.
///
/// Original: 0x0059EFC0 (thiscall, no stack words).
export!(thiscall, rw_0059efc0(out: u32) -> u32 {
    #[inline(always)]
    fn fmul(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    #[inline(always)]
    fn fadd(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }
    #[inline(always)]
    fn fsub(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) - core::hint::black_box(b)
    }
    #[inline(always)]
    fn fdiv(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) / core::hint::black_box(b)
    }
    /// x86 CVTTSS2SI semantics: truncate toward zero; NaN, infinities and
    /// out-of-range results yield 0x80000000 (Rust `as` saturates instead).
    #[inline(always)]
    fn cvtt_ss2si(x: f32) -> u32 {
        if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
            0x8000_0000
        } else {
            (x as i32) as u32
        }
    }
    unsafe {
        const MEM_BOUND: u32 = 0x5999_9980;
        const CACHED_LO: u32 = 0x01BB_5500;
        const CACHED_HI: u32 = 0x01BB_5504;
        const STATUS_LEN: u32 = 0x40;
        const INIT_FLAG: u32 = 0x01C9_9330;
        const CLASS_ARG: u32 = 0x00E6_E360;
        const GFACTOR: u32 = 0x018D_C2AC;
        const SHIFTKNOB: u32 = 0x0116_0EB4;
        const DECKNOB: u32 = 0x0116_0EB0;
        const F1INT: u32 = 0x0116_0E94;
        const OUT_A: u32 = 0x0116_0E8C;
        const TABLE: u32 = 0x0103_0B88;
        const ADD0: u32 = 0x0103_0B94;
        const ADD1: u32 = 0x0103_0B98;
        const CTR_LO: u32 = 0x017E_D998;
        const CTR_HI: u32 = 0x017E_D99C;
        const C1: u32 = 0x00FE_8714;
        const C2: u32 = 0x00FE_8840;
        const C3: u32 = 0x00FE_8888;
        const C4: u32 = 0x00FE_8A24;
        const C5: u32 = 0x00FE_88E8;
        const FILL_CALLEE: u32 = 1;
        const MEM_QUERY_CALLEE: u32 = 2;
        const GATEA_CALLEE: u32 = 3;
        const GATEB_CALLEE: u32 = 4;
        const PA_CALLEE: u32 = 5;
        const PB_CALLEE: u32 = 6;
        const PC_CALLEE: u32 = 7;
        const PD_CALLEE: u32 = 8;
        const PE_CALLEE: u32 = 9;
        const PF_CALLEE: u32 = 10;
        const INIT_CALLEE: u32 = 11;
        const CLASS_CALLEE: u32 = 12;
        const STAMP_CALLEE: u32 = 13;

        let mut w = [0u32; 4];
        let wptr = w.as_mut_ptr() as u32;
        let _ = callee_cdecl!(
            FILL_CALLEE, u32, wptr, wptr.wrapping_add(4), wptr.wrapping_add(8), wptr.wrapping_add(12));
        let cx = w[0];
        let dx = w[1];
        let prod = cx.wrapping_mul(dx);
        let mut edi = prod.wrapping_mul(9).wrapping_mul(4);
        let g = global::<u32>(GFACTOR).read();
        if g != 0 {
            edi = edi.wrapping_add(g.wrapping_mul(cx).wrapping_mul(dx).wrapping_mul(16));
        }
        let ga = callee_cdecl!(GATEA_CALLEE, u32,);
        let ebx: u32 = if (ga & 0xFF) != 0 {
            let gb = callee_cdecl!(GATEB_CALLEE, u32,);
            if gb > 1 { 0 } else { 1 }
        } else {
            0
        };
        let pa = callee_cdecl!(PA_CALLEE, u32,);
        let pb = callee_cdecl!(PB_CALLEE, u32, pa);
        let mut esi = ebx.wrapping_add(1).wrapping_mul(edi).wrapping_add(pb);
        let pc = callee_cdecl!(PC_CALLEE, u32,);
        let pd = callee_cdecl!(PD_CALLEE, u32, pc);
        esi = esi.wrapping_add(pd);
        let pe = callee_cdecl!(PE_CALLEE, u32,);
        let pf = callee_cdecl!(PF_CALLEE, u32, pe);
        esi = esi.wrapping_add(pf);
        let sh = global::<u32>(SHIFTKNOB).read();
        let mut eax = 0x80u32.wrapping_shl(sh & 31).wrapping_mul(0x80u32.wrapping_shl(sh & 31));
        let ck = global::<u32>(DECKNOB).read();
        let dec = ck.wrapping_sub(1);
        eax = eax.wrapping_mul(5);
        esi = esi.wrapping_add(eax.wrapping_mul(4));
        let neg = (dec as i32) < 0;
        eax = if neg {
            0x10
        } else {
            0x80u32.wrapping_shl(dec & 31)
        };
        let f1i = global::<u32>(F1INT).read();
        let ck3 = if neg { 0 } else { dec };
        eax = eax.wrapping_mul(eax);
        let ck4 = ck3.wrapping_add(1);
        let edx2 = 0x80u32.wrapping_shl(ck4 & 31);
        let c1 = f32::from_bits(global::<u32>(C1).read());
        let c2 = f32::from_bits(global::<u32>(C2).read());
        let c3 = f32::from_bits(global::<u32>(C3).read());
        let f1 = fadd(fmul(fmul((f1i as i32) as f32, c1), c2), c3);
        let edx3 = edx2.wrapping_mul(edx2);
        eax = eax.wrapping_add(edx3).wrapping_mul(3).wrapping_add(0x200000);
        edi = esi.wrapping_add(eax.wrapping_mul(4));
        let mut lo = global::<u32>(CACHED_LO).read();
        let mut hi = global::<u32>(CACHED_HI).read();
        let mut index = 0u32;
        if (lo | hi) == 0 {
            let mut status = [0u32; 16];
            status[0] = STATUS_LEN;
            let ok = callee_stdcall!(MEM_QUERY_CALLEE, u32, status.as_mut_ptr() as u32);
            if ok != 0 {
                lo = status[2];
                hi = status[3];
                global::<u32>(CACHED_LO).write(lo);
                global::<u32>(CACHED_HI).write(hi);
                if hi != 0 || lo > MEM_BOUND {
                    index = global::<u32>(OUT_A).read();
                }
            } else {
                index = global::<u32>(OUT_A).read();
            }
        } else if hi != 0 || lo > MEM_BOUND {
            index = global::<u32>(OUT_A).read();
        }
        let taddr = global::<u32>(TABLE) as u32;
        let tval = ((taddr.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned();
        let aval = tval
            .wrapping_shl(20)
            .wrapping_add(global::<u32>(ADD0).read())
            .wrapping_add(global::<u32>(ADD1).read());
        let t = (aval as f64) as f32;
        let total_lo = cvtt_ss2si(fmul(t, f1));
        let flag = global::<u32>(INIT_FLAG).read();
        if flag & 1 == 0 {
            global::<u32>(INIT_FLAG).write(flag | 1);
            let _ = callee_cdecl!(INIT_CALLEE, u32,);
            let _ = callee_cdecl!(CLASS_CALLEE, u32, relocated(CLASS_ARG));
        }
        let stamp = callee_cdecl!(STAMP_CALLEE, u64,);
        let glo = ((global::<u32>(CTR_HI).read() as u64) << 32) | global::<u32>(CTR_LO).read() as u64;
        let diff = glo.wrapping_sub(stamp).wrapping_sub(edi as u64);
        let fi = (diff as i64) as f32;
        let c4 = f32::from_bits(global::<u32>(C4).read());
        let c5 = f32::from_bits(global::<u32>(C5).read());
        let q = fdiv(fmul(fi, c4), (total_lo as i32) as f32);
        let outf = fsub(c5, q);
        (out as *mut u32).write_unaligned(outf.to_bits());
        total_lo.wrapping_add(edi)
    }
});
