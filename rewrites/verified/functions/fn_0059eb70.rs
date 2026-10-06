// original: 0x0059EB70 memsize_class_small (proposed)
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, export, global,
    relocated};

/// Set the two small memory-class limits (1 or 2) from total physical memory.
///
/// `arg1` (ECX) and `arg2` (EDX) are a divisor base and a scale numerator.
/// A one-time flag at `INIT_FLAG` (bit 0) guards two setup calls: when clear
/// the flag is set and the initializer callee plus a constant-argument
/// callee run. A mode byte at `MODE_BYTE` selects the class: 1 when the byte
/// is zero, else 2.
///
/// Memory sizing matches the sibling setter: the cached pair at
/// `CACHED_LO/HI`, refreshed by GlobalMemoryStatusEx when both are zero (a
/// 64-byte status block, first word 0x40, totals at words 2-3). A failed
/// query still computes the first limit.
///
/// The first limit (at `OUT_A`) is zero when memory is known-small (high
/// word clear and low word at or below `MEM_BOUND`, compared UNSIGNED);
/// otherwise `min(round_half_even(class * arg2 * 8 / (arg1 - 1)), class)`.
/// The second limit (at `OUT_B`) always computes:
/// `min(round_half_even(arg2 * 16 / (arg1 - 1)) + 1, 2)`. The 2 is constant:
/// the slot the clamp reads was stored from the class register BEFORE the
/// mode `cmove` selected 1 or 2, and is never updated, so it always holds
/// the initial 2 (the class affects only the first limit).
///
/// Finally the estimator callee is called with ECX pointing at that same
/// always-2 slot; it overwrites the slot with a float, returned in XMM0.
/// EAX on return is the estimator's answer.
///
/// Original: 0x0059EB70 (fastcall, no stack words; divisor `arg1 - 1` wraps,
/// so only `arg1 == 1` would divide by zero).
export!(fastcall, rw_0059eb70(arg1: u32, arg2: u32) -> f32 {
    /// Divide rounding half to even (remainder doubled with wrap, UNSIGNED
    /// below/above test, exact half resolved by quotient parity).
    #[inline(always)]
    fn scale_div(num: u32, den: u32) -> u32 {
        let q = num / den;
        let twice_rem = (num % den).wrapping_mul(2);
        if twice_rem < den {
            q
        } else if twice_rem > den {
            q.wrapping_add(1)
        } else if q & 1 == 1 {
            q.wrapping_add(1)
        } else {
            q
        }
    }
    /// Return `bits` as f32 through XMM0 (upper lanes zeroed), matching the
    /// original's `(an instruction of the original)`; the built export was disassembled to
    /// confirm (`movss` then `fld`).
    #[inline(always)]
    fn return_f32_bits(bits: u32) -> f32 {
        unsafe {
            let v = core::arch::x86::_mm_load_ss(&bits as *const u32 as *const f32);
            let f = core::arch::x86::_mm_cvtss_f32(v);
            core::hint::black_box(v);
            f
        }
    }
    unsafe {
        const MEM_BOUND: u32 = 0x5999_9980;
        const INIT_FLAG: u32 = 0x01C9_9330;
        const MODE_BYTE: u32 = 0x017E_D949;
        const CACHED_LO: u32 = 0x01BB_5500;
        const CACHED_HI: u32 = 0x01BB_5504;
        const OUT_A: u32 = 0x0116_0E8C;
        const OUT_B: u32 = 0x0116_0E90;
        const STATUS_LEN: u32 = 0x40;
        const CLASS_ARG: u32 = 0x00E6_E360;
        // [esp+0x14] is stored pre-cmove and never rewritten: always 2.
        const STALE_SLOT: u32 = 2;
        const INIT_CALLEE: u32 = 4;
        const CLASS_CALLEE: u32 = 5;
        const MEM_QUERY_CALLEE: u32 = 2;
        const ESTIMATE_CALLEE: u32 = 3;

        let flag = global::<u32>(INIT_FLAG).read();
        if flag & 1 == 0 {
            global::<u32>(INIT_FLAG).write(flag | 1);
            let _ = callee_cdecl!(INIT_CALLEE, u32,);
            let _ = callee_cdecl!(CLASS_CALLEE, u32, relocated(CLASS_ARG));
        }
        let class: u32 = if global::<u8>(MODE_BYTE).read() == 0 { 1 } else { 2 };
        let mut lo = global::<u32>(CACHED_LO).read();
        let mut hi = global::<u32>(CACHED_HI).read();
        let compute_first = if (lo | hi) == 0 {
            let mut status = [0u32; 16];
            status[0] = STATUS_LEN;
            let ok = callee_stdcall!(MEM_QUERY_CALLEE, u32, status.as_mut_ptr() as u32);
            if ok != 0 {
                lo = status[2];
                hi = status[3];
                global::<u32>(CACHED_LO).write(lo);
                global::<u32>(CACHED_HI).write(hi);
                hi != 0 || lo > MEM_BOUND
            } else {
                true
            }
        } else {
            hi != 0 || lo > MEM_BOUND
        };
        let den = arg1.wrapping_sub(1);
        if compute_first {
            let num = class.wrapping_mul(arg2).wrapping_mul(8);
            global::<u32>(OUT_A).write(core::cmp::min(scale_div(num, den), class));
        } else {
            global::<u32>(OUT_A).write(0);
        }
        let num2 = arg2.wrapping_mul(16);
        let q2 = scale_div(num2, den).wrapping_add(1);
        global::<u32>(OUT_B).write(core::cmp::min(q2, STALE_SLOT));
        let mut slot = STALE_SLOT;
        let _ = callee_thiscall!(ESTIMATE_CALLEE, u32, &mut slot as *mut u32 as u32);
        return_f32_bits(slot)
    }
});
