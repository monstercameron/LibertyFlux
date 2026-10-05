// original: 0x00c5d8f0 gang_driveby_sight_probe (proposed)

/// Probe the sight line for a gang-driveby task and score it, writing the
/// result to the task object.
///
/// `obj` is the task (`+0xb30` a helper object, `+0xd50` a table row,
/// `+0xdf8`/`+0xdfc` the float score and a zero word this function writes).
/// `flag`'s low byte enables the probe; when zero the outputs stay zeroed.
/// The helper's u16 at `+0x2e` indexes a global pointer table; the entry's
/// word at `+0xcc` points at a row array read at the task row. A row of -1
/// or a value of -1 ends the probe early. Otherwise callee 0 resolves the
/// value, callee 1 fills a three-float target point, and the helper's matrix
/// (`+0x20`, direction at `+0x20`/`+0x24`/`+0x28`) is scaled by 5.0 and
/// combined with the point into two triples that callee 3 consumes with the
/// helper pointer and constants 1, 1, 0. Callee 2 takes the shared struct
/// address; nothing it writes is read afterwards. When callee 3 succeeds it
/// fills a second point; the score is the dot product of the point
/// difference with the direction, clamped below at 0.6, minus a 0.85 bias.
/// Float operation order is the original's.
///
/// Original: 0x00c5d8f0 (stdcall, two stack words). No meaningful return.
lf_checker_rt::export!(stdcall, rw_00c5d8f0(obj: u32, flag: u32) -> u32 {
    unsafe {
        const CAL_RESOLVE: u32 = 0;
        const CAL_TARGET: u32 = 1;
        const CAL_PREP: u32 = 2;
        const CAL_PROBE: u32 = 3;
        const TABLE: u32 = 0x01295cd8;
        const SCALE: u32 = 0x00fe8ad8;
        const FLOOR: u32 = 0x00fe8858;
        const BIAS: u32 = 0x010494d4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
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

        wr32(obj.wrapping_add(0xdf8), 0);
        wr32(obj.wrapping_add(0xdfc), 0);
        if flag & 0xff == 0 {
            return 0;
        }
        let row = rd32(obj.wrapping_add(0xd50));
        if row == 0xffffffff {
            return 0;
        }
        let helper = rd32(obj.wrapping_add(0xb30));
        let idx = rd16(helper.wrapping_add(0x2e));
        let tab = lf_checker_rt::relocated(TABLE);
        let p0 = rd32(tab.wrapping_add(idx.wrapping_mul(4)));
        let p1 = rd32(p0.wrapping_add(0xcc));
        let val = rd32(p1.wrapping_add(row.wrapping_mul(4)));
        if val == 0xffffffff {
            return 0;
        }
        let resolved: u32 =
            lf_checker_rt::callee_thiscall!(CAL_RESOLVE, u32, helper, val);
        // Target point: the callee fills words 11..13 of this block.
        let mut tgt = [0u32; 15];
        lf_checker_rt::callee_thiscall!(
            CAL_TARGET,
            u32,
            tgt.as_mut_ptr() as u32,
            resolved
        );
        let p = [f32::from_bits(tgt[12]), f32::from_bits(tgt[13]), f32::from_bits(tgt[14])];
        let mat = rd32(helper.wrapping_add(0x20));
        let dir = [rdf(mat.wrapping_add(0x20)), rdf(mat.wrapping_add(0x24)), rdf(mat.wrapping_add(0x28))];
        let scale: f32 = rdf(lf_checker_rt::relocated(SCALE));
        let mut hi = [0.0f32; 3];
        let mut lo = [0.0f32; 3];
        for i in 0..3 {
            let s = mul(dir[i], scale);
            hi[i] = add(s, p[i]);
            lo[i] = sub(p[i], dir[i]);
        }
        let mut hi_b = [hi[0].to_bits(), hi[1].to_bits(), hi[2].to_bits()];
        let mut lo_b = [lo[0].to_bits(), lo[1].to_bits(), lo[2].to_bits()];
        let mut shared = [0u32; 7];
        lf_checker_rt::callee_cdecl!(CAL_PREP, u32,);
        let ok: u32 = lf_checker_rt::callee_cdecl!(
            CAL_PROBE,
            u32,
            helper,
            hi_b.as_mut_ptr() as u32,
            lo_b.as_mut_ptr() as u32,
            shared.as_mut_ptr() as u32,
            1,
            1,
            0
        );
        if ok & 0xff == 0 {
            return 0;
        }
        let q = [
            f32::from_bits(shared[4]),
            f32::from_bits(shared[5]),
            f32::from_bits(shared[6]),
        ];
        let t0 = mul(sub(q[1], p[1]), dir[1]);
        let t1 = mul(sub(q[0], p[0]), dir[0]);
        let t2 = mul(sub(q[2], p[2]), dir[2]);
        let mut dot = add(add(t0, t1), t2);
        let floor: f32 = rdf(lf_checker_rt::relocated(FLOOR));
        if !(dot > floor) {
            dot = floor;
        }
        let bias: f32 = rdf(lf_checker_rt::relocated(BIAS));
        wr32(obj.wrapping_add(0xdf8), sub(dot, bias).to_bits());
        0
    }
});
