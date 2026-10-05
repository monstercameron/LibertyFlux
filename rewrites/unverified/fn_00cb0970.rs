// original: 0x00cb0970 CTaskComplexMoveCrowdAroundLocation::vf20 (symbols)

/// Step a crowd-around-location complex task and return the active subtask.
///
/// `this` is the task, `edi` the owner object. Returns a full 32-bit value
/// in EAX: the subtask pointer kept in a frame slot, or callee 6's answer on
/// the replacement path (thiscall: `this` in ECX, one stack word, callee
/// pops 4).
///
/// Layout read: the task word at `+0x8` is the subtask object (vtable at
/// `+0`: slot `+0xC` is the status poll, slot `+0x14` the start gate; flag
/// dword at `+0xC` with bit 0 "started" and bit 1 set after starting; object
/// at `+0x14` whose word at `+0x24` is a direct code pointer), the task
/// vtable at `+0` (slot `+0x48` yields the replacement subtask), a waypoint
/// at `+0x20..+0x2C` with a shadow copy at `+0x40..+0x4C`, a position at
/// `+0x30..+0x38`, and flag bytes at `+0x59`/`+0x5A`. `edi+0x20` is a matrix
/// row read at `+0x30..+0x38`; `edi+0x224` points at a word counting down
/// from above 20. Two globals form a one-bit cache for a distance bound.
/// Callees 1-6 are direct (thiscall, stack arguments only); 7-10 are reached
/// through the objects above (7 = status poll, 8 = direct slot, 9 = start
/// gate, 10 = replacement fetch).
///
/// Algorithm: callee 1 validates the step; when it accepts (AL nonzero),
/// callees 2-5 run setup calls. The result slot starts as the subtask. When
/// the first poll answers exactly 0x384, an over-limit counter is cleared
/// (which also latches flag `0x59` and refreshes the shadow copy) and the
/// squared distance of position minus matrix row is compared against 16.0:
/// above it, or with flag `0x59` set, the start gate runs unless already
/// started (a zero answer keeps the old subtask, otherwise bit 1 is set) and
/// the replacement fetch refreshes the slot; otherwise the direct slot runs
/// with (owner, position address, 0). The second poll must answer exactly
/// 0x11A with flag `0x5A` clear or the slot is returned: with flag `0x59`
/// set the shadow-vs-waypoint distance above 1.0 continues, else the cached
/// bound (0.5625 on first fill) is compared against the position distance
/// and only a greater distance continues. The tail re-checks the started
/// bit, runs the start gate with (owner, 0, 0) unless started (a zero answer
/// returns the slot, otherwise bit 1 is set), then returns callee 6's answer
/// for (0x384, owner).
///
/// Edge cases: only AL of callees 1 and 9 is tested (upper bytes ignored);
/// the counter comparison is a signed 32-bit `> 20`; every float comparison
/// uses ordered `comiss` semantics (unordered takes the else side); the
/// three distance sums add their squares in their own orders (((dx+dy)+dz)
/// for the first two, ((dy+dx)+dz) for the cached one). All call arguments
/// are heap pointers or immediates, identical on both sides, so no argument
/// is skipped and ECX is compared on every callee.
lf_checker_rt::export!(thiscall, rw_00cb0970(this: u32, edi: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 0x08;
        const SUB_FLAGS: u32 = 0x0C;
        const STARTED_BIT: u32 = 0x01;
        const SET_BIT: u32 = 0x02;
        const DIRECT_OBJ: u32 = 0x14;
        const POLL_SLOT: u32 = 0x0C;
        const GATE_SLOT: u32 = 0x14;
        const DIRECT_SLOT: u32 = 0x24;
        const FETCH_SLOT: u32 = 0x48;
        const WAY_OFF: u32 = 0x20;
        const SHADOW_OFF: u32 = 0x40;
        const POS_OFF: u32 = 0x30;
        const FLAG_A: u32 = 0x59;
        const FLAG_B: u32 = 0x5A;
        const MAT_OFF: u32 = 0x20;
        const ROW_OFF: u32 = 0x30;
        const CTR_OBJ: u32 = 0x224;
        const CTR_OFF: u32 = 0x264;
        const CTR_LIMIT: i32 = 0x14;
        const POLL_FIRST: u32 = 0x384;
        const POLL_SECOND: u32 = 0x11A;
        const C_DIST: u32 = 0x00FE8B28;
        const C_ONE: u32 = 0x00FE88E8;
        const C_FILL: u32 = 0x00FE8848;
        const G_CACHE: u32 = 0x0171BF7C;

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
        #[inline(always)]
        unsafe fn poll(sub: u32) -> u32 {
            unsafe {
                let vt = rd32(sub);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt + POLL_SLOT) as usize);
                f(sub)
            }
        }
        #[inline(always)]
        unsafe fn gate(sub: u32, a0: u32, a1: u32, a2: u32) -> u32 {
            unsafe {
                let vt = rd32(sub);
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt + GATE_SLOT) as usize);
                f(sub, a0, a1, a2)
            }
        }

        if (lf_checker_rt::callee_thiscall!(1, u32, this, edi) & 0xFF) != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, edi, 1u32);
            lf_checker_rt::callee_thiscall!(3, u32, edi, this + WAY_OFF);
            lf_checker_rt::callee_thiscall!(4, u32, this, edi);
            lf_checker_rt::callee_thiscall!(5, u32, this, edi);
        }
        let sub = rd32(this + SUB_OFF);
        let mut result = sub;
        if poll(sub) == POLL_FIRST {
            let ctr = rd32(edi + CTR_OBJ);
            if (rd32(ctr + CTR_OFF) as i32) > CTR_LIMIT {
                wr32(ctr + CTR_OFF, 0);
                wr8(this + FLAG_A, 1);
                wr32(this + SHADOW_OFF, rd32(this + WAY_OFF));
                wrf(this + SHADOW_OFF + 4, rdf(this + WAY_OFF + 4));
                wrf(this + SHADOW_OFF + 8, rdf(this + WAY_OFF + 8));
                wr32(this + SHADOW_OFF + 12, rd32(this + WAY_OFF + 12));
            }
            let mat = rd32(edi + MAT_OFF);
            let dx = fsub(rdf(this + POS_OFF), rdf(mat + ROW_OFF));
            let dy = fsub(rdf(this + POS_OFF + 4), rdf(mat + ROW_OFF + 4));
            let dz = fsub(rdf(this + POS_OFF + 8), rdf(mat + ROW_OFF + 8));
            let d2 = fadd(fadd(fmul(dx, dx), fmul(dy, dy)), fmul(dz, dz));
            if d2 > rdf(lf_checker_rt::relocated(C_DIST)) || rd8(this + FLAG_A) != 0 {
                if (rd32(sub + SUB_FLAGS) & STARTED_BIT) == 0 {
                    if (gate(sub, edi, 1, 0) & 0xFF) != 0 {
                        wr32(sub + SUB_FLAGS, rd32(sub + SUB_FLAGS) | SET_BIT);
                        let vt = rd32(this);
                        let fetch: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                            rd32(vt + FETCH_SLOT) as usize,
                        );
                        result = fetch(this, edi);
                    }
                } else {
                    let vt = rd32(this);
                    let fetch: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(rd32(vt + FETCH_SLOT) as usize);
                    result = fetch(this, edi);
                }
            } else {
                let direct = rd32(sub + DIRECT_OBJ);
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(direct + DIRECT_SLOT) as usize);
                f(sub + DIRECT_OBJ, edi, this + POS_OFF, 0);
            }
        }
        if poll(sub) != POLL_SECOND || rd8(this + FLAG_B) != 0 {
            return result;
        }
        let go = if rd8(this + FLAG_A) != 0 {
            let dx = fsub(rdf(this + SHADOW_OFF), rdf(this + WAY_OFF));
            let dy = fsub(rdf(this + SHADOW_OFF + 4), rdf(this + WAY_OFF + 4));
            let dz = fsub(rdf(this + SHADOW_OFF + 8), rdf(this + WAY_OFF + 8));
            let dd = fadd(fadd(fmul(dx, dx), fmul(dy, dy)), fmul(dz, dz));
            dd > rdf(lf_checker_rt::relocated(C_ONE))
        } else {
            let g = lf_checker_rt::relocated(G_CACHE);
            let mut flags = rd32(g + 4);
            let bound = if (flags & 1) == 0 {
                let v = rdf(lf_checker_rt::relocated(C_FILL));
                flags |= 1;
                wr32(g + 4, flags);
                wrf(g, v);
                v
            } else {
                rdf(g)
            };
            let mat = rd32(edi + MAT_OFF);
            let dx = fsub(rdf(this + POS_OFF), rdf(mat + ROW_OFF));
            let dy = fsub(rdf(this + POS_OFF + 4), rdf(mat + ROW_OFF + 4));
            let dz = fsub(rdf(this + POS_OFF + 8), rdf(mat + ROW_OFF + 8));
            let d3 = fadd(fadd(fmul(dy, dy), fmul(dx, dx)), fmul(dz, dz));
            d3 > bound
        };
        if !go {
            return result;
        }
        if (rd32(sub + SUB_FLAGS) & STARTED_BIT) == 0 {
            if (gate(sub, edi, 0, 0) & 0xFF) == 0 {
                return result;
            }
            wr32(sub + SUB_FLAGS, rd32(sub + SUB_FLAGS) | SET_BIT);
        }
        lf_checker_rt::callee_thiscall!(6, u32, this, POLL_FIRST, edi)
    }
});
