// original: 0x00d859b0 scaled_lookup_blend_gate (proposed)

/// Gate a scaled table-vector lookup through a two-call blend pipeline and a
/// distance check, updating counter and state bytes on the target object.
///
/// `obj` points to the source object: 16-bit table indices at `+0xDE0` and
/// `+0xDE4` (low word selects a row base from the pointer table, high word a
/// 32-byte row within it), a mode byte at `+0xE6E`, a weight byte at `+0xE6F`,
/// a position block pointer at `+0x20`, and state bytes at `+0xF15`/`+0xF1C`.
/// `a1`/`a2`/`a3` are out-words zeroed on the fail path, `a4` is passed to
/// the report callee, and `a5` is the target object (counter at `+0x27`,
/// state at `+0x26`, flags at `+0x2B`).
///
/// Behaviour: either index `0xFFFF`, or a null table entry for either index,
/// takes the fail path (flag bit on the target, out-words zeroed). Otherwise
/// three signed 16-bit samples at row offsets `+0x14/+0x16/+0x18` are scaled
/// by 0.125, 0.125, 0.015625. When the mode byte is `0x12`, the second row is
/// expanded by callee 1, the difference of the two triples is transformed in
/// place by callee 2, and the scaled triple is added back; otherwise the
/// scaled pair is used as is. Callee 3 reports the pair with the six incoming
/// arguments. The target counter is clamped to 8, and the distance of the
/// pair from the position block's point at `+0x30/+0x34` is compared against
/// 4.0 (state `0x12`, sets `0x19` when below) or 2.0 (any other state, sets
/// flag bit, state 5, updates source state bytes, then callees 4 and 5).
/// An unordered (NaN) distance takes the far branch on both paths.
///
/// Original: 0x00d859b0 (cdecl, six stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00d859b0(obj: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0117_8284;
        const ROW_STRIDE: u32 = 32;
        const SCALE_XY: f32 = f32::from_bits(0x3e00_0000); // 0.125
        const SCALE_Z: f32 = f32::from_bits(0x3c80_0000); // 0.015625
        const NEAR_TRACK: f32 = f32::from_bits(0x4080_0000); // 4.0
        const NEAR_ACQ: f32 = f32::from_bits(0x4000_0000); // 2.0
        const CAL_EXPAND: u32 = 1;
        const CAL_BLEND: u32 = 2;
        const CAL_REPORT: u32 = 3;
        const CAL_STATE: u32 = 4;
        const CAL_NOTIFY: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd16s(a: u32) -> f32 {
            unsafe { (a as *const i16).read_unaligned() as i32 as f32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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

        let table = lf_checker_rt::relocated(TABLE);
        let lo_a = rd16(obj + 0x0de0);
        let hi_a = rd16(obj + 0x0de2);
        let lo_b = rd16(obj + 0x0de4);
        let hi_b = rd16(obj + 0x0de6);
        let fail = || -> u32 {
            unsafe {
                wr8(a5 + 0x2b, rd8(a5 + 0x2b) | 4);
                wr32(a3, 0);
                wr32(a2, 0);
                wr32(a1, 0);
            }
            0
        };
        if lo_b == 0xffff || lo_a == 0xffff {
            return fail();
        }
        let base_b = rd32(table.wrapping_add(lo_b.wrapping_mul(4)));
        if base_b == 0 {
            return fail();
        }
        let base_a = rd32(table.wrapping_add(lo_a.wrapping_mul(4)));
        if base_a == 0 {
            return fail();
        }
        let row_b = base_b.wrapping_add(hi_b.wrapping_mul(ROW_STRIDE));
        let v0 = mul(rd16s(row_b + 0x14), SCALE_XY);
        let v1 = mul(rd16s(row_b + 0x16), SCALE_XY);
        let v2 = mul(rd16s(row_b + 0x18), SCALE_Z);
        let (fa, fb);
        if rd8(obj + 0x0e6e) == 0x12 {
            let row_a = base_a.wrapping_add(hi_a.wrapping_mul(ROW_STRIDE));
            let mut out = [0.0f32; 3];
            lf_checker_rt::callee_thiscall!(CAL_EXPAND, u32, row_a, out.as_mut_ptr() as u32);
            let mut buf = [sub(v0, out[0]), sub(v1, out[1]), sub(v2, out[2])];
            lf_checker_rt::callee_thiscall!(CAL_BLEND, u32, buf.as_mut_ptr() as u32);
            fa = add(buf[0], v0);
            fb = add(buf[1], v1);
        } else {
            fa = v0;
            fb = v1;
        }
        let fe = rd8(obj + 0x0e6f) as i32 as f32;
        lf_checker_rt::callee_cdecl!(
            CAL_REPORT, u32, obj, 0, fa.to_bits(), fb.to_bits(),
            a1, a2, a3, a4, a5, fe.to_bits()
        );
        let count = rd8(a5 + 0x27);
        let count = if count < 8 { count } else { 8 };
        let tracking = rd8(a5 + 0x26) == 0x12;
        wr8(a5 + 0x27, count);
        let pos = rd32(obj + 0x20);
        let dx = sub(fa, rdf(pos + 0x30));
        let dy = sub(fb, rdf(pos + 0x34));
        let dist2 = add(mul(dy, dy), mul(dx, dx));
        let dist = dist2.sqrt();
        if tracking {
            if NEAR_TRACK > dist {
                wr8(a5 + 0x26, 0x19);
            }
        } else if NEAR_ACQ > dist {
            wr8(a5 + 0x2b, rd8(a5 + 0x2b) | 4);
            wr8(a5 + 0x26, 5);
            wr8(obj + 0x0f15, rd8(obj + 0x0f15) & 0xfe);
            lf_checker_rt::callee_thiscall!(CAL_STATE, u32, obj);
            wr8(obj + 0x0f1c, rd8(obj + 0x0f1c) | 0x80);
            lf_checker_rt::callee_cdecl!(CAL_NOTIFY, u32, obj, 0xffff_ffff);
        }
        0
    }
});
