// original: 0x00d85bf0 scaled_pair_energy_gate (proposed)

/// Gate a scaled table-vector pair through an energy call and a distance
/// check, updating counter and state bytes on the target object.
///
/// Layouts match the sibling `scaled_lookup_blend_gate`: table indices at
/// `obj+0xDE0`/`+0xDE4` (low word selects a row base, high word a 32-byte
/// row), a mode byte at `+0xE6E`, a weight byte at `+0xE6F`, a position block
/// pointer at `+0x20`, and state bytes at `+0xEF9`/`+0xF15`/`+0xF1D`.
/// `a1`/`a2`/`a3` are out-words zeroed on the fail path, `a4` is passed to
/// the report callee, and `a5` is the target object (counter at `+0x27`,
/// state at `+0x26`, flags at `+0x2B`).
///
/// Behaviour: either index `0xFFFF`, or a null table entry for either index,
/// takes the fail path (target state set to 5, out-words zeroed). Otherwise
/// the signed 16-bit pairs at row offsets `+0x14/+0x16` of both rows are
/// scaled by 0.125. When the mode byte is `0x11`, callee 1 scores the two
/// scaled pairs against the position point (its x87 result below 6.0 sets
/// target state `0x17`) and the first row's pair is reported; otherwise the
/// second row's pair is reported as is. Callee 2 reports the pair with the
/// six incoming arguments. The target counter is clamped to 8, and the
/// distance of the second row's pair from the position block's point at
/// `+0x30/+0x34` below 2.0 sets the target flag bit and the source state
/// bytes, then runs callees 3 and 4. An unordered (NaN) distance or energy
/// takes the far branch.
///
/// Original: 0x00d85bf0 (cdecl, six stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00d85bf0(obj: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0117_8284;
        const ROW_STRIDE: u32 = 32;
        const SCALE: f32 = f32::from_bits(0x3e00_0000); // 0.125
        const ENERGY_LT: f32 = f32::from_bits(0x40c0_0000); // 6.0
        const NEAR: f32 = f32::from_bits(0x4000_0000); // 2.0
        const CAL_ENERGY: u32 = 1;
        const CAL_REPORT: u32 = 2;
        const CAL_STATE: u32 = 3;
        const CAL_NOTIFY: u32 = 4;

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
        if lo_b == 0xffff || lo_a == 0xffff {
            wr8(a5 + 0x26, 5);
            wr32(a3, 0);
            wr32(a2, 0);
            wr32(a1, 0);
            return 0;
        }
        let base_b = rd32(table.wrapping_add(lo_b.wrapping_mul(4)));
        let base_a = rd32(table.wrapping_add(lo_a.wrapping_mul(4)));
        if base_b == 0 || base_a == 0 {
            wr8(a5 + 0x26, 5);
            wr32(a3, 0);
            wr32(a2, 0);
            wr32(a1, 0);
            return 0;
        }
        let row_a = base_a.wrapping_add(hi_a.wrapping_mul(ROW_STRIDE));
        let row_b = base_b.wrapping_add(hi_b.wrapping_mul(ROW_STRIDE));
        let ax = mul(rd16s(row_a + 0x14), SCALE);
        let ay = mul(rd16s(row_a + 0x16), SCALE);
        let bx = mul(rd16s(row_b + 0x14), SCALE);
        let by = mul(rd16s(row_b + 0x16), SCALE);
        let (fa, fb);
        if rd8(obj + 0x0e6e) == 0x11 {
            let pos = rd32(obj + 0x20);
            let px = rdf(pos + 0x30);
            let py = rdf(pos + 0x34);
            let s0 = [px, py, 0.0f32];
            let s1 = [ax, ay, 0.0f32];
            let s2 = [bx, by, 0.0f32];
            // Argument order: last pushed is arg0, so (s2, s1, s0).
            let energy: f32 = lf_checker_rt::callee_cdecl!(
                CAL_ENERGY, f32, s2.as_ptr() as u32, s1.as_ptr() as u32, s0.as_ptr() as u32
            );
            if ENERGY_LT > energy {
                wr8(a5 + 0x26, 0x17);
            }
            fa = ax;
            fb = ay;
        } else {
            fa = bx;
            fb = by;
        }
        let fe = rd8(obj + 0x0e6f) as i32 as f32;
        lf_checker_rt::callee_cdecl!(
            CAL_REPORT, u32, obj, 0, fa.to_bits(), fb.to_bits(),
            a1, a2, a3, a4, a5, fe.to_bits()
        );
        let count = rd8(a5 + 0x27);
        wr8(a5 + 0x27, if count < 8 { count } else { 8 });
        let pos = rd32(obj + 0x20);
        let dx = sub(bx, rdf(pos + 0x30));
        let dy = sub(by, rdf(pos + 0x34));
        // Note the square order: xmm0 (dy) first, then xmm1 (dx).
        let dist2 = add(mul(dy, dy), mul(dx, dx));
        let dist = dist2.sqrt();
        if NEAR > dist {
            wr8(a5 + 0x2b, rd8(a5 + 0x2b) | 4);
            wr8(obj + 0x0ef9, rd8(obj + 0x0ef9) | 1);
            wr8(obj + 0x0f15, rd8(obj + 0x0f15) & 0xfe);
            lf_checker_rt::callee_thiscall!(CAL_STATE, u32, obj);
            wr8(obj + 0x0f1d, rd8(obj + 0x0f1d) | 1);
            lf_checker_rt::callee_cdecl!(CAL_NOTIFY, u32, obj, 0xffff_ffff);
        }
        0
    }
});
