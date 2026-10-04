// original: 0x00655e20 pose_blend_apply (proposed)

/// Blend two keyframed tracks at a sample time and apply the result to a pose.
///
/// `this` is the blender object, `arg1` the source state and `arg2` the pose
/// being written. When `arg2` is null the function returns at once; otherwise
/// it resets the pose through the first helper and returns if the enable flag
/// at `this + 0x111` is clear. It then samples two tracks: each has a base
/// pointer (`+0xc0`, `+0xe8`) and a 16-bit count (`+0xc4`, `+0xec`) over rows
/// of 0x30 bytes holding a key float, four base floats at `+0x10` and four
/// slope floats at `+0x20`. The sample time comes from the object at
/// `arg1 + 0x260`, offset `+0x280`. Each track takes the first row whose key
/// is not below the time (an ordered float comparison: NaN keys never match),
/// or the last row when none matches, or row zero when the count is below 2,
/// and interpolates four outputs as slope * (time - key) + base in that
/// order. When the object at `this + 0x100` is non-null, both output rows are
/// passed to the second helper (which rewrites them through out-pointers)
/// with a mode word of 4 and 5 and the sample time; otherwise the
/// interpolated rows are kept. The second row, scaled by the radians constant,
/// drives three calls into the row-rotation family (angles in xmm1, object at
/// `arg2 + 0x10`), after storing the flag byte from `this + 0x112` at
/// `arg2 + 0`. The first row lands at `arg2 + 0x40`, scaled by the factor at
/// `arg1 + 0x1e8` after a call into the sixth helper with `arg1 + 0xa0`, and
/// the last elements of both rows, scaled by the same factor, land at
/// `arg2 + 0x50` and `arg2 + 0x54`. There is no meaningful return value.
///
/// Original: 0x00655e20 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00655e20(this: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x30;
        const KEY_OFF: u32 = 0x280;
        const TRACK1_BASE: u32 = 0xc0;
        const TRACK1_COUNT: u32 = 0xc4;
        const TRACK2_BASE: u32 = 0xe8;
        const TRACK2_COUNT: u32 = 0xec;
        const SUB_OBJ: u32 = 0x100;
        const ENABLE_FLAG: u32 = 0x111;
        const POSE_FLAG: u32 = 0x112;
        const DEG_TO_RAD: u32 = 0xfe8728;

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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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

        /// First row whose key is not below `key` (ordered comparison), the
        /// last row when the scan runs out, or row zero for counts below 2.
        unsafe fn find_row(base: u32, count: u32, key: f32) -> u32 {
            unsafe {
                let mut idx = 1u32;
                if count > 1 {
                    let mut cursor = base.wrapping_add(ROW_STRIDE);
                    loop {
                        if rdf(cursor) >= key {
                            break;
                        }
                        idx += 1;
                        if idx >= count {
                            break;
                        }
                        cursor = cursor.wrapping_add(ROW_STRIDE);
                    }
                }
                base.wrapping_add(idx.wrapping_sub(1).wrapping_mul(ROW_STRIDE))
            }
        }

        /// Four slope * (time - key) + base outputs for the row.
        unsafe fn interp(row: u32, key: f32) -> [f32; 4] {
            unsafe {
                let t = sub(key, rdf(row));
                [
                    add(mul(rdf(row + 0x20), t), rdf(row + 0x10)),
                    add(mul(rdf(row + 0x24), t), rdf(row + 0x14)),
                    add(mul(rdf(row + 0x28), t), rdf(row + 0x18)),
                    add(mul(rdf(row + 0x2c), t), rdf(row + 0x1c)),
                ]
            }
        }

        if arg2 == 0 {
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, arg2);
        if rd8(this + ENABLE_FLAG) == 0 {
            return 0;
        }
        let si = rd32(arg1 + 0x260);
        let key = rdf(si + KEY_OFF);
        let count1 = rd16(this + TRACK1_COUNT) as u32;
        let row1 = find_row(rd32(this + TRACK1_BASE), count1, key);
        let mut b1 = interp(row1, key);
        let count2 = rd16(this + TRACK2_COUNT) as u32;
        let row2 = find_row(rd32(this + TRACK2_BASE), count2, key);
        let mut b2 = interp(row2, key);
        let sub = rd32(this + SUB_OBJ);
        if sub != 0 {
            let out_this = arg1.wrapping_add(0x1a4);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                2,
                u32,
                sub,
                0,
                b1.as_mut_ptr() as u32,
                4,
                key.to_bits(),
                out_this
            );
            let key2 = rdf(rd32(arg1 + 0x260) + KEY_OFF);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                2,
                u32,
                sub,
                0,
                b2.as_mut_ptr() as u32,
                5,
                key2.to_bits(),
                out_this
            );
        }
        let rad = f32::from_bits(*lf_checker_rt::global::<u32>(DEG_TO_RAD));
        let pose_flag = rd8(this + POSE_FLAG);
        let pose_obj = arg2.wrapping_add(0x10);
        wr8(arg2, pose_flag);
        let _: u32 =
            lf_checker_rt::callee_thiscall!(3, u32, pose_obj, mul(b2[0], rad).to_bits());
        let _: u32 =
            lf_checker_rt::callee_thiscall!(4, u32, pose_obj, mul(b2[1], rad).to_bits());
        let _: u32 =
            lf_checker_rt::callee_thiscall!(5, u32, pose_obj, mul(b2[2], rad).to_bits());
        wrf(arg2 + 0x40, b1[0]);
        wrf(arg2 + 0x44, b1[1]);
        wrf(arg2 + 0x48, b1[2]);
        let mult = rdf(arg1 + 0x1e8);
        let s0 = mul(mult, b1[0]);
        let pusharg = arg1.wrapping_add(0xa0);
        wrf(arg2 + 0x40, s0);
        wrf(arg2 + 0x44, mul(mult, b1[1]));
        wrf(arg2 + 0x48, mul(mult, b1[2]));
        let _: u32 = lf_checker_rt::callee_thiscall!(6, u32, pose_obj, pusharg);
        wrf(arg2 + 0x50, mul(rdf(arg1 + 0x1e8), b2[3]));
        wrf(arg2 + 0x54, mul(rdf(arg1 + 0x1e8), b1[3]));
        0
    }
});
