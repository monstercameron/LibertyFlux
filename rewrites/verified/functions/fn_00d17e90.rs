// original: 0x00D17E90 slide_cover_pose_update (proposed)

/// Advance a cover-slide task by one step: solve the slide, blend a pose
/// sample and tick the slide timer.
///
/// `this` is the task, `arg0` the ped. Two frameless float helpers
/// (callees 1 and 2, argument and result in XMM0) map the task rate at
/// +0x60 to a pair whose scaled difference and sum seed the solver input;
/// an optional notifier (callee 3, same frameless shape) runs when the
/// ped's slide handle (+0xd68) is set. The slide solver (callee 4, fourteen
/// arguments including the ped handle or null, a code callback or null
/// selected by bits of +0x90, and the heap and frame vectors) rejects the
/// step by returning zero. Otherwise a lookup (callee 5) and a setup
/// (callee 6) run, the ped-eye offset minus the current pose sample feeds
/// the blend (callee 7), whose four out-parameter words become the new
/// pose sample at +0x40. A solver block (callee 8, thiscall) yields four
/// floats, the first two of which drive an x87 helper (callee 9) whose
/// result lands at +0x64; a scaled integer query (callee 10) below 0.9
/// additionally routes +0x64 through a second x87 helper (callee 11) after
/// subtracting pi. Finally the timer at +0x70 advances by 0.05 (clamped at
/// 1.0), the reserve bit of +0x94 is cleared and the function returns 1.
///
/// Original: 0x00D17E90 (thiscall, one stack word, returns a byte in al).
lf_checker_rt::export!(thiscall, rw_00D17E90(this: u32, arg0: u32) -> u32 {
    unsafe {
        const C_RATE_LO: u32 = 0xfe88bc;
        const C_RATE_SCALE: u32 = 0xfe8684;
        const C_PI: u32 = 0xfe8aa0;
        const C_TICK: u32 = 0xfe876c;
        const C_ONE: u32 = 0xfe88e8;
        const C_TWO_PI_BITS: u32 = 0x40c90fdb;
        const CALLBACK_ADDR: u32 = 0xd1f640;
        const CALL_HELPER_A: u32 = 1;
        const CALL_HELPER_B: u32 = 2;
        const CALL_NOTIFY: u32 = 3;
        const CALL_SOLVER: u32 = 4;
        const CALL_LOOKUP: u32 = 5;
        const CALL_SETUP: u32 = 6;
        const CALL_BLEND: u32 = 7;
        const CALL_BLOCK: u32 = 8;
        const CALL_X87A: u32 = 9;
        const CALL_QUERY: u32 = 10;
        const CALL_X87B: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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

        let rate_bits = rd32(this + 0x60);
        let a = f32::from_bits(lf_checker_rt::callee_cdecl!(CALL_HELPER_A, u32, rate_bits));
        let b = f32::from_bits(lf_checker_rt::callee_cdecl!(CALL_HELPER_B, u32, rate_bits));
        let v1 = sub(mul(b, 0.0), a);
        let v2 = add(mul(a, 0.0), b);
        let inner = rd32(arg0 + 0xd68);
        if inner != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(CALL_NOTIFY, u32, b.to_bits());
        }
        let (cb, argex) = if rd8(this + 0x90) & 7 == 2 {
            (lf_checker_rt::relocated(CALLBACK_ADDR), arg0)
        } else {
            (0, 0)
        };
        let wsx = (rd16(this + 0x7a) as i16) as i32 as u32;
        let inner20 = rd32(arg0 + 0x20);
        let mut step = [v1.to_bits(), v2.to_bits(), 0u32];
        let dret: u32 = lf_checker_rt::callee_cdecl!(
            CALL_SOLVER,
            u32,
            this + 0x40,
            this + 0x20,
            step.as_ptr() as u32,
            C_TWO_PI_BITS,
            0x3f800000,
            rd32(this + 0x68),
            0,
            1,
            inner20 + 0x30,
            wsx,
            0,
            0,
            cb,
            argex
        );
        if dret == 0 {
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(CALL_LOOKUP, u32, arg0, dret);
        let _: u32 = lf_checker_rt::callee_thiscall!(CALL_SETUP, u32, inner, arg0);
        let e20 = rd32(arg0 + 0x20);
        let q0 = sub(rdf(e20 + 0x38), rdf(this + 0x48));
        let q2 = sub(rdf(e20 + 0x30), rdf(this + 0x40));
        let q1 = sub(rdf(e20 + 0x34), rdf(this + 0x44));
        let g_in = [q2.to_bits(), q1.to_bits(), q0.to_bits(), 0u32];
        let mut g_out = [0u32; 4];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            CALL_BLEND,
            u32,
            inner,
            g_in.as_ptr() as u32,
            this + 0x40,
            g_out.as_mut_ptr() as u32
        );
        let mut h_scratch = [0u32; 4];
        let blk: u32 = lf_checker_rt::callee_thiscall!(
            CALL_BLOCK,
            u32,
            inner,
            h_scratch.as_ptr() as u32,
            g_in.as_ptr() as u32
        );
        let b0 = f32::from_bits(rd32(blk));
        let b1 = f32::from_bits(rd32(blk + 4));
        let b2 = f32::from_bits(rd32(blk + 8));
        let _b3 = rd32(blk + 12);
        let q: f32 = lf_checker_rt::callee_cdecl!(CALL_X87A, f32, b0.to_bits(), b1.to_bits(), 0, 0);
        wrf(this + 0x64, q);
        let j: u32 = lf_checker_rt::callee_thiscall!(CALL_QUERY, u32, inner);
        let sc = mul((j as i32) as f32, f32::from_bits(rd32(lf_checker_rt::relocated(C_RATE_SCALE))));
        if f32::from_bits(rd32(lf_checker_rt::relocated(C_RATE_LO))) > sc {
            let qb = sub(rdf(this + 0x64), f32::from_bits(rd32(lf_checker_rt::relocated(C_PI))));
            let q2: f32 = lf_checker_rt::callee_cdecl!(CALL_X87B, f32, qb.to_bits());
            wrf(this + 0x64, q2);
        }
        wrf(this + 0x40, f32::from_bits(g_out[0]));
        wrf(this + 0x44, f32::from_bits(g_out[1]));
        wrf(this + 0x48, f32::from_bits(g_out[2]));
        wrf(this + 0x4c, f32::from_bits(g_out[3]));
        let t70 = add(rdf(this + 0x70), f32::from_bits(rd32(lf_checker_rt::relocated(C_TICK))));
        wrf(this + 0x70, t70);
        if t70 > f32::from_bits(rd32(lf_checker_rt::relocated(C_ONE))) {
            wrf(this + 0x70, 1.0);
        }
        wr8(this + 0x94, rd8(this + 0x94) & 0xfe);
        let _ = b2;
        1
    }
});
