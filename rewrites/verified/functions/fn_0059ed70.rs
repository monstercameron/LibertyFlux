// original: 0x0059ED70 memsize_class_ab (proposed)
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, export, global};

/// Set the two memory-class limits from the machine's total physical memory.
///
/// `arg1` (ECX) and `arg2` (EDX) are a divisor base and a scale numerator.
/// A gate callee selects the scale pair: when its SIGNED answer exceeds 2
/// the scales are (24, 30), otherwise (21, 10).
///
/// Total physical memory comes from the cached pair at `CACHED_LO/HI`; when
/// both are zero the function queries GlobalMemoryStatusEx (a 64-byte status
/// block whose first word is 0x40, total bytes at words 2-3) and caches the
/// answer on success. When the query fails the first limit is computed
/// anyway (the failure branch jumps straight to the computation).
///
/// The first limit (at `OUT_A`) is zero when memory is known-small (high
/// word clear and low word at or below `MEM_BOUND`, compared UNSIGNED);
/// otherwise it is `min(round_half_even(scale_a * arg2 / (arg1 - 1)) + 16,
/// scale_a)`. The second limit (at `OUT_B`) always computes:
/// `min(round_half_even(scale_b * arg2 / (arg1 - 1)) + 8, scale_b)`.
///
/// Finally the estimator callee is called with ECX pointing at a slot
/// holding the unclamped second value; it overwrites the slot with a float,
/// which is returned in XMM0. EAX on return is the estimator's answer.
///
/// Original: 0x0059ED70 (fastcall, no stack words; divisor `arg1 - 1` wraps,
/// so only `arg1 == 1` would divide by zero).
export!(fastcall, rw_0059ed70(arg1: u32, arg2: u32) -> f32 {
    /// Divide rounding half to even, as the original's `div` + doubled
    /// remainder compare does (remainder doubled with wrap, UNSIGNED
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
    /// original's `(an instruction of the original)`. The forced vector load plus black_box
    /// keeps the value live in XMM0 across the ST0 return sequence; the
    /// built export was disassembled to confirm (`movss` then `fld`).
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
        const GATE_BOUND: i32 = 2;
        const SCALE_A_HI: u32 = 24;
        const SCALE_B_HI: u32 = 30;
        const SCALE_A_LO: u32 = 21;
        const SCALE_B_LO: u32 = 10;
        const MEM_BOUND: u32 = 0x5999_9980;
        const CACHED_LO: u32 = 0x01BB_5500;
        const CACHED_HI: u32 = 0x01BB_5504;
        const OUT_A: u32 = 0x0116_0E94;
        const OUT_B: u32 = 0x0116_0E98;
        const STATUS_LEN: u32 = 0x40;
        const GATE_CALLEE: u32 = 1;
        const MEM_QUERY_CALLEE: u32 = 2;
        const ESTIMATE_CALLEE: u32 = 3;

        let gate = callee_cdecl!(GATE_CALLEE, u32,);
        let (scale_a, scale_b) = if (gate as i32) > GATE_BOUND {
            (SCALE_A_HI, SCALE_B_HI)
        } else {
            (SCALE_A_LO, SCALE_B_LO)
        };
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
            let q = scale_div(scale_a.wrapping_mul(arg2), den);
            global::<u32>(OUT_A).write(core::cmp::min(q.wrapping_add(16), scale_a));
        } else {
            global::<u32>(OUT_A).write(0);
        }
        let q2 = scale_div(scale_b.wrapping_mul(arg2), den);
        let unclamped = q2.wrapping_add(8);
        global::<u32>(OUT_B).write(core::cmp::min(unclamped, scale_b));
        let mut slot = unclamped;
        let _ = callee_thiscall!(ESTIMATE_CALLEE, u32, &mut slot as *mut u32 as u32);
        return_f32_bits(slot)
    }
});
