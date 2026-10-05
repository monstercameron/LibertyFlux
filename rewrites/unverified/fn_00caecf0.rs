// original: 0x00caecf0 move_goal_satisfied_check (proposed)

/// Decide whether a ped move task's goal counts as satisfied.
///
/// `this` is the task object, `p` points to three floats (the target point),
/// `obj` is the owner object. Returns 1 in AL when the goal is satisfied, 0
/// otherwise (thiscall: `this` in ECX, the rest on the stack, callee pops 8).
///
/// Layout read: `obj+0x20` is a matrix row whose words at `+0x30..+0x3C` are
/// the current position; `obj+0xA80` is a state object (dword at `+0x48`,
/// flag byte at `+0x50`, vtable at `+0`) whose virtual slot at `+0x34` (callee
/// 2) yields a float; `obj+0x29C` bit 2 selects a clamped limit; the task
/// carries two floats at `+0xA0`/`+0xA4`, a position cache at `+0x50..+0x5C`
/// and a mode word at `+0xC4` (bit 6 vetoes acceptance, bit 1 skips the final
/// test, bits 11..14 are refreshed from callee 1).
///
/// Algorithm: a `blocked` flag is latched when the state flag byte has bit
/// 0x20 set or the state dword is not -1. Callee 1 (direct, thiscall, three
/// stack words: `obj`, `p`, and a pointer to a local holding bits 11..14 of
/// the mode word) answers accept/reject in AL and rewrites the local. When it
/// accepts, bit 6 is clear and nothing is blocked, the function returns 1
/// without touching the task. Otherwise the local is merged into mode bits
/// 11..14 and the geometry runs: dx/dy/dz-abs of target minus position, the
/// squared planar distance, the hook float added to `+0xA0` and squared, and
/// the dot of (target minus cache) with (dx, dy). The position row is then
/// published into the cache (`+0x50` and `+0x5C` as bit copies, `+0x54` and
/// `+0x58` as floats). A limit (task `+0xA4`, clamped up to the constant 4.0
/// when the select bit is set) larger than dz-abs, with bit 6 clear and bit 1
/// clear-or-negative-dot, accepts when unblocked. Bit 1 set rejects. Else
/// callee 3 (virtual slot `+0xEC` on `obj`, one out-pointer argument, returns
/// a pointer to two floats) scales an offset by the global float; the
/// residual dot decides: a positive (or unordered) residual rejects, bit 6
/// rejects, otherwise unblocked accepts.
///
/// Edge cases: only AL of callee 1's answer is read (upper bytes ignored);
/// every float comparison uses ordered `comiss` semantics (unordered behaves
/// as false, except the final residual test which rejects on unordered too);
/// a NaN limit falls back to the constant; all arithmetic runs in the
/// original's operand order. The out-pointer given to callee 3 addresses
/// uninitialized frame scratch whose contents neither side observes.
lf_checker_rt::export!(thiscall, rw_00caecf0(this: u32, p: u32, obj: u32) -> u32 {
    unsafe {
        const STATE_OFF: u32 = 0xA80;
        const STATE_WORD: u32 = 0x48;
        const STATE_FLAGS: u32 = 0x50;
        const BLOCK_BIT: u8 = 0x20;
        const MODE_OFF: u32 = 0xC4;
        const MODE_SHIFT: u32 = 11;
        const MODE_BITS: u32 = 0x7800;
        const VETO_BIT: u32 = 0x40;
        const SKIP_BIT: u32 = 0x02;
        const MAT_OFF: u32 = 0x20;
        const ROW_OFF: u32 = 0x30;
        const CACHE_X: u32 = 0x50;
        const CACHE_Y: u32 = 0x54;
        const CACHE_Z: u32 = 0x58;
        const CACHE_W: u32 = 0x5C;
        const HOOK_BASE: u32 = 0xA0;
        const LIMIT_OFF: u32 = 0xA4;
        const SELECT_OFF: u32 = 0x29C;
        const SELECT_BIT: u8 = 0x04;
        const HOOK_SLOT: u32 = 0x34;
        const PAIR_SLOT: u32 = 0xEC;
        const ABS_MASK: u32 = 0x00FE8F80;
        const CLAMP_CONST: u32 = 0x00FE8AB8;
        const SCALE_GLOBAL: u32 = 0x011735BC;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let state = rd32(obj + STATE_OFF);
        let blocked =
            (rd8(state + STATE_FLAGS) & BLOCK_BIT) != 0 || rd32(state + STATE_WORD) != 0xFFFF_FFFF;
        let mut out = (rd32(this + MODE_OFF) >> MODE_SHIFT) & 0xF;
        let ok = lf_checker_rt::callee_thiscall!(1, u32, this, obj, p, &mut out as *mut u32 as u32);
        let c4 = rd32(this + MODE_OFF);
        if (ok & 0xFF) != 0 && (c4 & VETO_BIT) == 0 && !blocked {
            return 1;
        }
        wr32(this + MODE_OFF, (c4 & !MODE_BITS) | ((out << MODE_SHIFT) & MODE_BITS));
        let mat = rd32(obj + MAT_OFF);
        let dz = fsub(rdf(mat + ROW_OFF + 8), rdf(p + 8));
        let mask = rd32(lf_checker_rt::relocated(ABS_MASK));
        let zabs = f32::from_bits(dz.to_bits() & mask);
        let dx = fsub(rdf(p), rdf(mat + ROW_OFF));
        let dy = fsub(rdf(p + 4), rdf(mat + ROW_OFF + 4));
        let sumsq = fadd(fmul(dy, dy), fmul(dx, dx));
        let row = mat + ROW_OFF;
        let hook_vt = rd32(state);
        let hook: extern "thiscall" fn(u32) -> f32 =
            unsafe { core::mem::transmute(rd32(hook_vt + HOOK_SLOT) as usize) };
        let got = hook(state);
        let s = fadd(rdf(this + HOOK_BASE), got);
        let sq = fmul(s, s);
        let t1 = fmul(fsub(rdf(p), rdf(this + CACHE_X)), dx);
        let dl = sq > sumsq;
        let dot = fadd(fmul(fsub(rdf(p + 4), rdf(this + CACHE_Y)), dy), t1);
        let dh = 0.0f32 > dot;
        wr32(this + CACHE_X, rd32(row));
        wrf(this + CACHE_Y, rdf(row + 4));
        wrf(this + CACHE_Z, rdf(row + 8));
        wr32(this + CACHE_W, rd32(row + 12));
        let mut lim = rdf(this + LIMIT_OFF);
        if (rd8(obj + SELECT_OFF) & SELECT_BIT) != 0 {
            let c = rdf(lf_checker_rt::relocated(CLAMP_CONST));
            if !(lim > c) {
                lim = c;
            }
        }
        if dl {
            let c4b = rd32(this + MODE_OFF);
            if (c4b & VETO_BIT) == 0 && lim > zabs && ((c4b & SKIP_BIT) == 0 || dh) && !blocked {
                return 1;
            }
        }
        if (rd32(this + MODE_OFF) & SKIP_BIT) != 0 {
            return 0;
        }
        let pair_vt = rd32(obj);
        let pair: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(pair_vt + PAIR_SLOT) as usize) };
        let mut scratch = [0u32; 2];
        let r = pair(obj, &mut scratch as *mut u32 as u32);
        let g = rdf(lf_checker_rt::relocated(SCALE_GLOBAL));
        let q0 = fmul(g, rdf(r));
        let q1 = fmul(rdf(r + 4), g);
        let s0 = fadd(rdf(row), q0);
        let s1 = fadd(rdf(row + 4), q1);
        let d0 = fsub(rdf(p), s0);
        let d1 = fsub(rdf(p + 4), s1);
        let rr = fadd(fmul(d1, dy), fmul(d0, dx));
        if !(0.0f32 >= rr) {
            return 0;
        }
        if (rd32(this + MODE_OFF) & VETO_BIT) != 0 {
            return 0;
        }
        if !blocked {
            1
        } else {
            0
        }
    }
});
