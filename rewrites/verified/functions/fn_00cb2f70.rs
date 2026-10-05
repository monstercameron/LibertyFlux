// original: 0x00cb2f70 CTaskComplexMoveWander::vf19

/// Roll wander parameters, then order the matching subtask.
///
/// `this` is the complex task: old slot at `+0x44`, rate at `+0x40`, range
/// at `+0xa4`/`+0xa8`, base heading at `+0x2c`, mode at `+0x24`, flags at
/// `+0xb0`, scratch buffer at `+0x64`. `ped` is the ped. Callee 1 releases
/// the old slot, callee 2 is the random helper, callee 3 maps the heading
/// (returning its float in ST0), callee 4 applies the rolled parameters,
/// callee 5 fetches the buffer object for the global at `0x179d114`,
/// callee 6 builds the subtask.
///
/// Behaviour: a nonzero old slot is released through callee 1 and cleared.
/// When the rate is exactly zero (either sign) the rolled value mixes the
/// range with a random draw, `((r * K1) * (hi - lo)) + lo` with the fixed
/// scale `K1 = 0x38000100`; any other rate, NaN included, takes the range
/// low end unchanged (this reproduces the original's `lahf` parity test on
/// the rate). Callee 3 maps base plus rate. Mode 2 orders `0x3ae` at once.
/// Otherwise callee 4 takes (ped, mapped, rolled); flag bit 6 is cleared
/// and, when bit 5 is clear, `0x11a` is ordered. When bit 5 is set it is
/// cleared, callee 5 fetches the buffer (a null answer faults on the count
/// read exactly like the original), the ped's position rows scaled by 4 and
/// shifted are appended at slot `(count + 1) * 16` when the count is below
/// 8 (it is always 0, just zeroed) with a zero stack word in the last lane
/// under the checker's zero stack fill, and `0x389` is ordered. Returns
/// callee 6's answer.
///
/// Float order is the original's scalar-SSE order.
///
/// Original: 0x00cb2f70 (thiscall, one stack word; callees 1-3 are cdecl
/// with one, none and one word, callee 3 returning f32 in ST0; callees 4-6
/// are thiscall with three, none and two words).
lf_checker_rt::export!(thiscall, rw_00cb2f70(this: u32, ped: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x44;
        const RATE: u32 = 0x40;
        const LO: u32 = 0xa4;
        const HI: u32 = 0xa8;
        const BASE: u32 = 0x2c;
        const MODE: u32 = 0x24;
        const FLAGS: u32 = 0xb0;
        const BUF_OUT: u32 = 0x64;
        const PED_MAT: u32 = 0x20;
        const ORDER_DIRECT: u32 = 0x3ae;
        const ORDER_PLAIN: u32 = 0x11a;
        const ORDER_WANDER: u32 = 0x389;
        const K1: f32 = f32::from_bits(0x3800_0100);
        const FOUR: f32 = 4.0;
        const GLOBAL_BUF: u32 = 0x179d114;
        const CALLEE_REL: u32 = 1;
        const CALLEE_RAND: u32 = 2;
        const CALLEE_MAP: u32 = 3;
        const CALLEE_APPLY: u32 = 4;
        const CALLEE_FETCH: u32 = 5;
        const CALLEE_TASK: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let slot = rd32(this + SLOT);
        if slot != 0 {
            lf_checker_rt::callee_cdecl!(CALLEE_REL, u32, slot);
            wr32(this + SLOT, 0);
        }
        let rate = rdf(this + RATE);
        let r = if rate != 0.0 {
            rdf(this + LO)
        } else {
            let lo = rdf(this + LO);
            let hi = rdf(this + HI);
            let n = lf_checker_rt::callee_cdecl!(CALLEE_RAND, u32,) as i32 as f32;
            add(mul(mul(n, K1), sub(hi, lo)), lo)
        };
        let t: f32 = lf_checker_rt::callee_cdecl!(CALLEE_MAP, f32, add(rdf(this + BASE), rate).to_bits());
        if rd32(this + MODE) == 2 {
            return lf_checker_rt::callee_thiscall!(CALLEE_TASK, u32, this, ped, ORDER_DIRECT);
        }
        lf_checker_rt::callee_thiscall!(CALLEE_APPLY, u32, this, ped, t.to_bits(), r.to_bits());
        wr32(this + FLAGS, rd32(this + FLAGS) & 0xffff_ffbf);
        let fl = rd32(this + FLAGS);
        if fl & 0x20 == 0 {
            return lf_checker_rt::callee_thiscall!(CALLEE_TASK, u32, this, ped, ORDER_PLAIN);
        }
        wr32(this + FLAGS, fl & 0xffff_ffdf);
        let g = (lf_checker_rt::global::<u32>(GLOBAL_BUF) as *const u32).read_unaligned();
        let c = lf_checker_rt::callee_thiscall!(CALLEE_FETCH, u32, g);
        if c != 0 {
            wr32(c, 0);
        }
        wr32(this + BUF_OUT, c);
        let m = rd32(ped + PED_MAT);
        let x3 = mul(rdf(m + 0x10), FOUR);
        let x1 = mul(rdf(m + 0x14), FOUR);
        let x2 = mul(rdf(m + 0x18), FOUR);
        let o0 = add(rdf(m + 0x30), x3);
        let o3 = add(rdf(m + 0x34), x1);
        let cnt = rd32(c);
        let o1 = add(rdf(m + 0x38), x2);
        if (cnt as i32) < 8 {
            let e = c.wrapping_add((cnt.wrapping_add(1)).wrapping_mul(16));
            wrf(e, o0);
            wrf(e + 4, o3);
            wrf(e + 8, o1);
            // The original copies an uninitialised stack word here; the
            // contract pins the stack fill to zero, so this is zero.
            wrf(e + 12, 0.0);
            wr32(c, cnt.wrapping_add(1));
        }
        lf_checker_rt::callee_thiscall!(CALLEE_TASK, u32, this, ped, ORDER_WANDER)
    }
});
