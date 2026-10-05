// original: 0x00A1DCB0 task_blend_m33_and_maybe_scale (proposed)

/// Blend a 3x3 transform by two weights, with an optional scale correction.
///
/// `task` is the task object and `out` a caller-provided 16-byte vector
/// slot. The function first resolves a parameter block: it calls the virtual
/// slot at `+0xa0` of `task` (thiscall, no arguments); a zero answer reads
/// the block from `task+0x100`, otherwise the slot is called again and the
/// virtual slot at `+0xe0` of that answer is called, whose answer is the
/// block. The block feeds two fixed callees: `parse(block[4], 0)` (cdecl,
/// two arguments) whose answer is passed to `weights(task, answer)`
/// (thiscall, one argument), which returns the two blend weights at
/// `+0x34`/`+0x38` of its answer.
///
/// The core blends rows of the matrix at `task+0x20` (each output is
/// `(row_a * w0 + row_b * 0) + row_c * w1 + row_d`, with the exact grouping
/// below and a zero factor kept as a multiply, so NaN and infinite inputs
/// propagate as in the original):
/// - `out[0] = ((M[0x10]*w0 + M[0]*0) + M[0x20]*w1) + M[0x30]`
/// - `out[4] = ((M[0x14]*w0 + M[4]*0) + M[0x24]*w1) + M[0x34]`
/// - `out[8] = ((M[0x18]*w0 + M[8]*0) + M[0x28]*w1) + M[0x38]`
/// The fourth word the original writes is a copy of its own uninitialised
/// stack scratch (the compiler reloads a dead slot above the pushed
/// arguments); under the checker's defined stack fill that word is 0, which
/// is what this rewrite stores.
///
/// Then the select callee runs as `select(task[0x224]+0x44, 0xfe)`
/// (thiscall, one argument). A null answer ends the function. Otherwise the
/// dword at `answer[0x10]` is read (null ends it too), then `r` at
/// `+0xc` of that; `r - 0x69` above `0x13` ends it, else a 20-byte case map
/// selects the scale arm (cases 0, 3, 18, 19) or the end. The scale arm
/// adds `M[0x10..0x18]` times the tuned factor (-0.15) into `out[0..8]`.
///
/// The return value is whatever the last step left in `eax` (the matrix
/// pointer after the scale arm, the case-map byte or `r - 0x69` on the
/// early exits, 0 after a null answer); it is deterministic on every path.
///
/// Original: 0x00A1DCB0 (stdcall, two stack words).
lf_checker_rt::export!(stdcall, rw_00A1DCB0(task: u32, out: u32) -> u32 {
    unsafe {
        const PARAM_BLOCK: u32 = 0x100;
        const MATRIX: u32 = 0x20;
        const SELECT_BASE: u32 = 0x224;
        const SELECT_DELTA: u32 = 0x44;
        const SELECT_ARG: u32 = 0xfe;
        const SLOT_PARAM: u32 = 0xa0;
        const SLOT_BLOCK: u32 = 0xe0;
        const WEIGHT0: u32 = 0x34;
        const WEIGHT1: u32 = 0x38;
        const CASE_BASE: u32 = 0x69;
        const CASE_MAX: u32 = 0x13;
        const FACTOR_VA: u32 = 0x00e8bfe0;
        const PARSE: u32 = 1;
        const WEIGHTS: u32 = 2;
        const SELECT: u32 = 3;
        const V_PARAM: u32 = 20;
        const V_BLOCK: u32 = 21;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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

        // Resolve the parameter block through the two virtual slots.
        let vparam: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(task) + SLOT_PARAM) as usize);
        let first = vparam(task);
        let block = if first == 0 {
            rd32(task + PARAM_BLOCK)
        } else {
            let second = vparam(task);
            let vblock: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(second) + SLOT_BLOCK) as usize);
            vblock(second)
        };
        // Push order in the original is `0` then `block[4]`, so the callee's
        // first argument is `block[4]`.
        let parsed = lf_checker_rt::callee_cdecl!(PARSE, u32, rd32(block + 4), 0u32);
        let w = lf_checker_rt::callee_thiscall!(WEIGHTS, u32, task, parsed);
        let w0 = rdf(w + WEIGHT0);
        let w1 = rdf(w + WEIGHT1);
        let m = rd32(task + MATRIX);
        let zero = 0.0f32;
        wrf(out, add(add(add(mul(rdf(m + 0x10), w0), mul(rdf(m), zero)), mul(rdf(m + 0x20), w1)), rdf(m + 0x30)));
        wrf(out + 4, add(add(add(mul(rdf(m + 0x14), w0), mul(rdf(m + 4), zero)), mul(rdf(m + 0x24), w1)), rdf(m + 0x34)));
        wrf(out + 8, add(add(add(mul(rdf(m + 0x18), w0), mul(rdf(m + 8), zero)), mul(rdf(m + 0x28), w1)), rdf(m + 0x38)));
        // The original's fourth word is its own uninitialised scratch (see
        // the doc comment); the proof defines that fill as 0.
        wr32(out + 12, 0);
        let sel = lf_checker_rt::callee_thiscall!(SELECT, u32, rd32(task + SELECT_BASE).wrapping_add(SELECT_DELTA), SELECT_ARG);
        if sel == 0 {
            return 0;
        }
        let r1 = rd32(sel + 0x10);
        if r1 == 0 {
            return 0;
        }
        let idx = rd32(r1 + 0xc).wrapping_sub(CASE_BASE);
        if idx > CASE_MAX {
            return idx;
        }
        // Case map: 0 for cases {0, 3, 18, 19} (scale arm), 1 otherwise.
        let arm = matches!(idx, 0 | 3 | 18 | 19);
        if !arm {
            return 1;
        }
        let k = f32::from_bits(rd32(lf_checker_rt::relocated(FACTOR_VA)));
        wrf(out, add(rdf(out), mul(rdf(m + 0x10), k)));
        wrf(out + 4, add(rdf(out + 4), mul(rdf(m + 0x14), k)));
        wrf(out + 8, add(rdf(out + 8), mul(rdf(m + 0x18), k)));
        m
    }
});
