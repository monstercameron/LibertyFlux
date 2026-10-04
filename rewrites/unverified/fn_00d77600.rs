// original: 0x00d77600 CRenderPhaseMirrorReflection::vf4

/// Run one mirror-reflection render phase over a scene snapshot: reduce the
/// phase parameters, publish them, run the four-stage filter chain, then
/// project and finish.
///
/// `this` points to the render-phase object (descriptor at `+0xb0`, second
/// descriptor at `+0x4a0`, three filter seeds at `+0x510`/`+0x514`/`+0x518`,
/// parameter block pointer at `+0x940`, status byte at `+0x1d`) and `arg`
/// points to the scene snapshot (view at `+0x10`, selector at `+0x538`,
/// status source at `+0x558`). Behaviour:
/// - Bind both descriptors to the snapshot view.
/// - When the parameter block exists: seed an eight-word frame buffer with
///   the three filter seeds (the rest zero) and reduce it through the
///   reducer, which also fills the buffer's second half; publish the block's
///   selector (`+0x30`) and count (`+0x38`) to the globals.
/// - When the block exists with a nonzero selector and a positive count:
///   latch count into `+0x7c` and selector into `+0x78`, then run the
///   four-stage filter chain over the buffer's second half (a describe call,
///   a describe-with-descriptor call, a five-argument combine of selector,
///   block, buffer, phase and zero, and a finish call). Otherwise run the
///   fallback call with argument 0.
/// - Project the current matrix (four words) through the projector with the
///   phase object, run the six-argument setter (0, 0, 1, 1, 0, 1) with the
///   descriptor, and when the scale flag is set project the fresh matrix
///   again with its fourth word scaled by the global factor (seed times
///   factor, in that order) through the scaler with the descriptor.
/// - Copy bit 2 of the snapshot's status byte into the phase status byte and
///   return the snapshot pointer with its low byte replaced by the full
///   status byte (the status load overwrites the accumulator's low byte).
///
/// Original: 0x00d77600 (thiscall, one stack argument, returns the argument
/// with its low byte replaced).
lf_checker_rt::export!(thiscall, rw_00d77600(this: u32, arg: u32) -> u32 {
    unsafe {
        const DESC0: u32 = 0xb0;
        const DESC1: u32 = 0x4a0;
        const VIEW: u32 = 0x10;
        const PARAMS: u32 = 0x940;
        const SEED0: u32 = 0x510;
        const SEED1: u32 = 0x514;
        const SEED2: u32 = 0x518;
        const PARAM_SELECTOR: u32 = 0x30;
        const PARAM_COUNT: u32 = 0x38;
        const LATCH_SELECTOR: u32 = 0x78;
        const LATCH_COUNT: u32 = 0x7c;
        const STATUS: u32 = 0x1d;
        const SNAP_SELECTOR: u32 = 0x538;
        const SNAP_STATUS: u32 = 0x558;
        const G_SELECTOR: u32 = 0x166DA70;
        const G_COUNT: u32 = 0x166DA74;
        const G_SCALE_FLAG: u32 = 0x1797694;
        const G_SCALE: u32 = 0xFE8D94;
        const ONE: f32 = 1.0;
        const BIND: u32 = 1;
        const REDUCE: u32 = 2;
        const DESCRIBE: u32 = 3;
        const DESCRIBE_WITH: u32 = 4;
        const COMBINE: u32 = 5;
        const FINISH: u32 = 6;
        const FALLBACK: u32 = 7;
        const CURRENT_MATRIX: u32 = 8;
        const PROJECT: u32 = 9;
        const SET_SIX: u32 = 10;
        const SCALE_PROJECT: u32 = 11;
        const FRESH_MATRIX: u32 = 12;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        lf_checker_rt::callee_thiscall!(BIND, u32, this.wrapping_add(DESC0), arg.wrapping_add(VIEW));
        lf_checker_rt::callee_thiscall!(BIND, u32, this.wrapping_add(DESC1), arg.wrapping_add(VIEW));

        let params = rd32(this.wrapping_add(PARAMS));
        // Frame buffer: three seeds then zeros. The reducer fills words 4..8
        // on both sides (scripted out-parameters); the filter chain observes
        // words 4..8 only.
        let mut buf = [0u32; 8];
        buf[0] = rd32(this.wrapping_add(SEED0));
        buf[1] = rd32(this.wrapping_add(SEED1));
        buf[2] = rd32(this.wrapping_add(SEED2));
        if params != 0 {
            lf_checker_rt::callee_thiscall!(
                REDUCE,
                u32,
                params,
                rd32(arg.wrapping_add(SNAP_SELECTOR)),
                buf.as_mut_ptr() as u32,
            );
            wr32(lf_checker_rt::relocated(G_SELECTOR), rd32(params.wrapping_add(PARAM_SELECTOR)));
            wr32(lf_checker_rt::relocated(G_COUNT), rd32(params.wrapping_add(PARAM_COUNT)));
        }

        let block = rd32(this.wrapping_add(PARAMS));
        if block != 0
            && rd32(block.wrapping_add(PARAM_SELECTOR)) != 0
            && (rd32(block.wrapping_add(PARAM_COUNT)) as i32) > 0
        {
            wr32(
                block.wrapping_add(LATCH_COUNT),
                rd32(block.wrapping_add(PARAM_COUNT)),
            );
            let fresh = rd32(this.wrapping_add(PARAMS));
            wr32(
                fresh.wrapping_add(LATCH_SELECTOR),
                rd32(fresh.wrapping_add(PARAM_SELECTOR)),
            );
            let half = unsafe { buf.as_mut_ptr().add(4) as u32 };
            lf_checker_rt::callee_thiscall!(DESCRIBE, u32, half);
            lf_checker_rt::callee_thiscall!(DESCRIBE_WITH, u32, half, this.wrapping_add(DESC0));
            lf_checker_rt::callee_cdecl!(
                COMBINE,
                u32,
                rd32(block.wrapping_add(PARAM_SELECTOR)),
                block,
                half,
                this,
                0u32,
            );
            lf_checker_rt::callee_thiscall!(FINISH, u32, half);
        } else {
            lf_checker_rt::callee_cdecl!(FALLBACK, u32, 0u32);
        }

        let desc = this.wrapping_add(DESC0);
        let mat = lf_checker_rt::callee_cdecl!(CURRENT_MATRIX, u32,);
        let mut proj = [rd32(mat), rd32(mat.wrapping_add(4)), rd32(mat.wrapping_add(8)), rd32(mat.wrapping_add(12))];
        lf_checker_rt::callee_thiscall!(PROJECT, u32, this, proj.as_mut_ptr() as u32);
        lf_checker_rt::callee_thiscall!(SET_SIX, u32, desc, 0u32, 0u32, ONE.to_bits(), ONE.to_bits(), 0u32, ONE.to_bits());

        if rd8(lf_checker_rt::relocated(G_SCALE_FLAG)) != 0 {
            let mat2 = lf_checker_rt::callee_cdecl!(FRESH_MATRIX, u32,);
            let scaled = mul(rdf(mat2.wrapping_add(12)), rdf(lf_checker_rt::relocated(G_SCALE)));
            let mut proj2 = [
                rd32(mat2),
                rd32(mat2.wrapping_add(4)),
                rd32(mat2.wrapping_add(8)),
                scaled.to_bits(),
            ];
            lf_checker_rt::callee_thiscall!(SCALE_PROJECT, u32, desc, proj2.as_mut_ptr() as u32);
        }

        let source = rd8(arg.wrapping_add(SNAP_STATUS));
        unsafe { ((this.wrapping_add(STATUS)) as *mut u8).write((source >> 2) & 1) };
        // The status load overwrites the accumulator's low byte, so the
        // returned snapshot pointer carries the status byte in its low byte.
        (arg & 0xffff_ff00) | (source as u32)
    }
});
