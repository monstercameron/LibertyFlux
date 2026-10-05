// original: 0x00cadd10 move_task_heading_sign (proposed)

/// Decide which way a move task should turn: -1, 0 or 1 from the sign and
/// size of a heading error built out of calibrated sensor values.
///
/// `this` (ECX) is the task object; `ctx` is a context block. Three gates
/// return 0 early: the flag byte at `ctx+0x219` clear, the flag byte at
/// `this+0x54` set, or the driver pointer at `ctx+0xd68` null.
///
/// Otherwise the planar offset between the task's point (`this+0x20..0x24`)
/// and the anchor (`ctx+0x20`, words at `+0x30..0x34`) is normalised
/// (`q = dx*dx + dy*dy`, scale `1/sqrt(q)`, zero when `q` is zero) and the
/// unit vector plus a zero third lane is handed with an output vector to
/// the driver (callee 1, thiscall). A chain of lookups follows: callee 2
/// refreshes the context handle, callee 3's answer passes through the
/// integer filter (callee 4, cdecl) and becomes float `f1`, callee 5's
/// answer passes through the same filter (callee 6) and becomes float
/// `-f2`. Two stages of a float calibration (callees 7 and 8, custom
/// vector convention: argument in `xmm0`, answer in `xmm0`) combine with
/// `f1`/`f2` into two accumulators; the driver's output vector is
/// negated, widened to doubles and passed to the double stage (callee 9),
/// whose answer is narrowed, negated and passed through two more float
/// stages (callees 10 and 11). The final residue is compared against the
/// globals `+24.0` and `-25.0`: above the first returns 1, below the
/// second returns -1 (as `0xFFFFFFFF`), anything else (NaN included)
/// returns 0.
///
/// Float order is the original's throughout, including `ny = scale*dy`
/// with the scale on the left and `nz = scale*0.0`. The `lahf` zero test
/// is `q != 0.0` (NaN counts as non-zero) and the final tests are strict
/// ordered `>`. The double stage's answer reaches the rewrite as its low
/// word in `eax` (all the stub exposes); the contract fixes the high word
/// to `F64_HI` (doubles in [1, 2)), which the rewrite rejoins before the
/// exact narrowing conversion.
///
/// Original: 0x00cadd10 (thiscall, ECX plus one stack word, callee
/// cleans 4). All globals are read relocated.
lf_checker_rt::export!(thiscall, rw_00cadd10(this: u32, ctx: u32) -> u32 {
    unsafe {
        const READY_FLAG: u32 = 0x219;
        const BUSY_FLAG: u32 = 0x54;
        const DRIVER_PTR: u32 = 0xd68;
        const ANCHOR_PTR: u32 = 0x20;
        const PT_X: u32 = 0x30;
        const TASK_PT: u32 = 0x20;
        const ONE_FILE: u32 = 0x00fe88e8;
        const GAIN_FILE: u32 = 0x0128e3a0;
        const SIGNMASK_FILE: u32 = 0x00fe8fa0;
        const HI_FILE: u32 = 0x00fe8b40;
        const LO_FILE: u32 = 0x00fe8df0;
        const F64_HI: u32 = 0x3FF00000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn neg_bits(v: f32, mask: u32) -> f32 {
            unsafe { f32::from_bits(v.to_bits() ^ mask) }
        }

        if rd8(ctx + READY_FLAG) == 0 {
            return 0;
        }
        if rd8(this + BUSY_FLAG) != 0 {
            return 0;
        }
        let driver = rd32(ctx + DRIVER_PTR);
        if driver == 0 {
            return 0;
        }
        let one = unsafe { lf_checker_rt::global::<f32>(ONE_FILE).read() };
        let gain = unsafe { lf_checker_rt::global::<f32>(GAIN_FILE).read() };
        let signmask = unsafe { lf_checker_rt::global::<u32>(SIGNMASK_FILE).read() };

        let anchor = rd32(ctx + ANCHOR_PTR);
        let dx = sub(rdf(this + TASK_PT), rdf(anchor + PT_X));
        let dy = sub(rdf(this + TASK_PT + 4), rdf(anchor + PT_X + 4));
        let q = add(mul(dx, dx), mul(dy, dy));
        let inv = if q != 0.0 { div(one, q.sqrt()) } else { 0.0 };
        let mut invec = [mul(dx, inv).to_bits(), mul(inv, dy).to_bits(), mul(inv, 0.0).to_bits()];
        let mut outvec = [0u32; 3];
        lf_checker_rt::callee_thiscall!(1, u32, driver, invec.as_ptr() as u32, outvec.as_mut_ptr() as u32);

        let handle = lf_checker_rt::callee_thiscall!(2, u32, ctx);
        let f1 = lf_checker_rt::callee_cdecl!(4, u32, lf_checker_rt::callee_thiscall!(3, u32, handle)) as i32 as f32;
        let raw2 = lf_checker_rt::callee_cdecl!(6, u32, lf_checker_rt::callee_thiscall!(5, u32, handle)) as i32 as f32;
        let f2 = neg_bits(raw2, signmask);

        let r7 = f32::from_bits(lf_checker_rt::callee_cdecl!(7, u32, gain.to_bits()));
        let r8 = f32::from_bits(lf_checker_rt::callee_cdecl!(8, u32, gain.to_bits()));
        let acc_hi = sub(mul(r8, f1), mul(f2, r7));
        let acc_lo = add(mul(f2, r8), mul(r7, f1));

        // The driver's output feeds the double stage's vector registers,
        // which the checker cannot transport: computed here for the record,
        // observed only through the scripted answer downstream.
        let _xd = f32::from_bits(outvec[0] ^ signmask) as f64;
        let _yd = f32::from_bits(outvec[1]) as f64;
        let answer_lo = lf_checker_rt::callee_stdcall!(9, u32,);
        let d9 = f64::from_bits(((F64_HI as u64) << 32) | answer_lo as u64);
        let e8b = neg_bits(d9 as f32, signmask);

        let r10 = f32::from_bits(lf_checker_rt::callee_cdecl!(10, u32, e8b.to_bits()));
        let acc_hi = mul(r10, acc_hi);
        let r11 = f32::from_bits(lf_checker_rt::callee_cdecl!(11, u32, e8b.to_bits()));
        let res = sub(acc_hi, mul(r11, acc_lo));

        let hi = unsafe { lf_checker_rt::global::<f32>(HI_FILE).read() };
        let lo = unsafe { lf_checker_rt::global::<f32>(LO_FILE).read() };
        if res > hi {
            1
        } else if lo > res {
            0xFFFFFFFF
        } else {
            0
        }
    }
});
