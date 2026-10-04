// original: 0x00D26A90 ped_task_aim_update (proposed)

/// Aim-update for a targeted ped task: measure range, open a handle, resolve
/// two keyed slots to positions, and steer the task toward them.
///
/// `obj` is the task object; `obj+0x20` points at a position source whose
/// floats at `+0x30/+0x34/+0x38` are copied to a scratch probe and handed to
/// the range-measurement callee. The run proceeds only when the run flag is
/// clear, the two tick counters agree, the game state is not `0x12`, the
/// measured range is strictly below 900.0, and the manager opens a handle.
///
/// The two slots come from keyed lookups (`0x4b3`, `0x36a1`) resolved to
/// values and then to position records. From the two positions the function
/// forms the midpoint and a scaled direction: the squared length is compared
/// against half the z extent, and the scale is that product itself on exact
/// equality, else the reciprocal of the length. A matrix builder fills two
/// scratch buffers, twelve words are copied with holes (indices 3, 7, 11 and
/// 15 stay zero), two combiners run, the handle is attached and registered,
/// and a final eleven-word report call runs.
///
/// Edge cases: every early exit returns a deterministic value (the tick on a
/// state mismatch, the range bits on a range failure, zero on a null
/// handle) except the shut-down-flag exit, whose value is the caller's
/// leftover in eax; the contract holds that flag clear so that path never
/// runs. NaN inputs take the reciprocal-length path, never the equality
/// path; the direction scaling multiplies x as `dx*s` but y and z as `s*d`,
/// matching the original's operand order exactly.
///
/// Original: 0x00D26A90 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00D26A90(obj: u32) -> u32 {
    unsafe {
        const RUN_FLAG: u32 = 0x011F7060;
        const TICK_A: u32 = 0x012088B4;
        const TICK_B: u32 = 0x00F1C040;
        const GAME_STATE: u32 = 0x01037720;
        const RANGE_LIMIT: u32 = 0x00E9CAE0;
        const HALF: u32 = 0x00FE8830;
        const ONE: u32 = 0x00FE88E8;
        const MANAGER: u32 = 0x01394D60;
        const TASK_NAME: u32 = 0x00EE19C0;
        const KEY_A: u32 = 0x4b3;
        const KEY_B: u32 = 0x36a1;
        const AWAY_STATE: u32 = 0x12;
        const OFF_LINK: u32 = 0x20;
        const OFF_X: u32 = 0x30;
        const OFF_Y: u32 = 0x34;
        const OFF_Z: u32 = 0x38;
        const C_MEASURE: u32 = 1;
        const C_FIND: u32 = 2;
        const C_OPEN: u32 = 3;
        const C_KEY: u32 = 4;
        const C_LOOKUP: u32 = 5;
        const C_SLOT: u32 = 6;
        const C_BUILD: u32 = 7;
        const C_XFORM: u32 = 8;
        const C_COMB_A: u32 = 9;
        const C_COMB_B: u32 = 10;
        const C_ATTACH: u32 = 11;
        const C_SET: u32 = 12;
        const C_REG: u32 = 13;
        const C_DONE: u32 = 14;
        const C_REPORT: u32 = 15;
        /// (source word in the build buffer, dest word in the copy buffer).
        const COPY: [(usize, usize); 12] = [
            (4, 0), (5, 1), (6, 2),
            (8, 4), (9, 5), (10, 6),
            (12, 8), (13, 9), (14, 10),
            (16, 12), (17, 13), (18, 14),
        ];

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        if g32(RUN_FLAG) == 1 {
            // Indeterminate in the original (leftover eax); never taken here.
            return 0;
        }
        let tick = g32(TICK_A);
        if tick != g32(TICK_B) {
            return tick;
        }
        if g32(GAME_STATE) == AWAY_STATE {
            return tick;
        }

        let link = rd32(obj + OFF_LINK);
        let probe = [rdf(link + OFF_X), rdf(link + OFF_Y), rdf(link + OFF_Z)];
        let range: f32 = lf_checker_rt::callee_cdecl!(C_MEASURE, f32, probe.as_ptr() as u32);
        if !(gf(RANGE_LIMIT) > range) {
            return range.to_bits();
        }

        let name = lf_checker_rt::callee_cdecl!(C_FIND, u32, lf_checker_rt::relocated(TASK_NAME), 0);
        let mgr = lf_checker_rt::relocated(MANAGER);
        let h = lf_checker_rt::callee_thiscall!(C_OPEN, u32, mgr, name, 0, 0);
        if h == 0 {
            return 0;
        }
        let k1 = lf_checker_rt::callee_thiscall!(C_KEY, u32, obj, KEY_A);
        let t1 = lf_checker_rt::callee_cdecl!(C_LOOKUP, u32, k1, KEY_A);
        let k2 = lf_checker_rt::callee_thiscall!(C_KEY, u32, obj, KEY_B);
        let t2 = lf_checker_rt::callee_cdecl!(C_LOOKUP, u32, k2, KEY_B);
        let o1 = lf_checker_rt::callee_thiscall!(C_SLOT, u32, obj, t1);
        let o2 = lf_checker_rt::callee_thiscall!(C_SLOT, u32, obj, t2);

        let (o1x, o1y, o1z) = (rdf(o1 + OFF_X), rdf(o1 + OFF_Y), rdf(o1 + OFF_Z));
        let (o2x, o2y, o2z) = (rdf(o2 + OFF_X), rdf(o2 + OFF_Y), rdf(o2 + OFF_Z));
        let dx = sub(o2x, o1x);
        let dy = sub(o2y, o1y);
        let dz = sub(o2z, o1z);
        let half = gf(HALF);
        let zx = mul(dx, half);
        let zy = mul(dy, half);
        let zz = mul(dz, half);
        let mut p2 = [0u32; 20];
        p2[0] = add(o1x, zx).to_bits();
        p2[1] = add(zy, o1y).to_bits();
        p2[2] = add(o1z, zz).to_bits();
        let sqlen = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
        // Skip (scale = zz) exactly when the original's ucomiss+lahf+test
        // finds ordered equality; NaN always takes the reciprocal path.
        let s = if sqlen == zz {
            zz
        } else {
            div(gf(ONE), core::hint::black_box(sqlen).sqrt())
        };
        let dir = [mul(dx, s), mul(s, dy), mul(s, dz)];
        let mut p3 = [0u32; 8];
        lf_checker_rt::callee_cdecl!(
            C_BUILD, u32,
            p3.as_mut_ptr() as u32,
            p2.as_mut_ptr() as u32,
            dir.as_ptr() as u32,
            1
        );
        lf_checker_rt::callee_thiscall!(C_XFORM, u32, p2.as_mut_ptr().offset(4) as u32, o1);
        let mut mc = [0u32; 16];
        for (src, dst) in COPY {
            mc[dst] = p2[src];
        }
        lf_checker_rt::callee_thiscall!(
            C_COMB_A, u32,
            p2.as_mut_ptr().offset(4) as u32,
            mc.as_mut_ptr() as u32
        );
        let mut m4 = [0u32; 8];
        lf_checker_rt::callee_thiscall!(C_XFORM, u32, m4.as_mut_ptr() as u32, p3.as_mut_ptr() as u32);
        lf_checker_rt::callee_thiscall!(
            C_COMB_B, u32,
            m4.as_mut_ptr() as u32,
            p2.as_mut_ptr().offset(4) as u32
        );
        lf_checker_rt::callee_thiscall!(C_ATTACH, u32, h, t2);
        lf_checker_rt::callee_thiscall!(C_SET, u32, h, m4.as_mut_ptr() as u32);
        lf_checker_rt::callee_thiscall!(C_REG, u32, mgr, h, obj, 0);
        lf_checker_rt::callee_thiscall!(C_DONE, u32, h);
        lf_checker_rt::callee_cdecl!(
            C_REPORT, u32, name, obj, 0, 0, 0, 0, 0, 0xFFFF_FFFF, 0xFFFF_FFFF, 0, 0
        )
    }
});
