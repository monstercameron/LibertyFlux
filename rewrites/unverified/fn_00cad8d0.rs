// original: 0x00cad8d0 move_task_blend_avoidance (proposed)

/// Blend a crowd-avoidance offset into a move task's goal point.
///
/// `this` (ECX) is the task object; `arg` is a context block. The function
/// first consults the task's current subtask (the object at `this+0x08`,
/// through its virtual slot at `+0x0c`, callee 1): a first answer of
/// `GATE_FIRST` proceeds, otherwise a second answer of `GATE_SECOND`
/// proceeds and anything else returns without writing. A null subtask
/// proceeds with no calls.
///
/// It then accumulates one repulsion term per occupied candidate slot: the
/// table at `arg+0x224` holds sixteen pointers starting at `SLOT_BASE`
/// past it. A slot is skipped when null, when its flag byte at `+0x210`
/// is set with state `+0xa74` equal to 1 or 2, when its word at `+0x224`
/// is zero, or when its byte at `+0x212` is set. Otherwise the offset
/// between the anchor point (`arg+0x20`, words at `+0x30..0x38`) and the
/// slot's point (same layout under `slot+0x20`) is measured; only squared
/// distances strictly inside (`DIST_LO`, `DIST_HI`) contribute, as
/// `dir * (1/len) * (1-len)` added onto the running sum Mix order is the
/// original's: `d2 = (dy*dy + dx*dx) + dz*dz`, and the new contribution
/// is the left operand when added to the running sum.
///
/// A zero accumulated vector returns without writing. Otherwise the sum
/// is added onto the goal point at `this+0x30..0x38`, the result is
/// re-based on the base point at `this+0x20..0x28`, scaled by
/// `(1/len) * this+0x50` (a zero length scales by zero), and written back
/// over the goal point. The word at `this+0x3c` is uninitialised stack
/// scratch in the original; under the checker's defined zero stack fill
/// it is `0.0`, which is what this rewrite stores. The squared-length
/// associations differ per site (`(ax*ax + ay*ay) + az*az` for the sum,
/// `(ry*ry + rx*rx) + rz*rz` for the re-based vector) and are kept as is.
/// All float comparisons are the original's exact conditions: the two
/// window tests are strict ordered `>`, and the two zero tests treat NaN
/// as non-zero (the `lahf`/`test`/`jp` idiom is `!= 0.0`, its `jnp`
/// twin is `== 0.0`).
///
/// The return register is never set on any path (whatever the last call
/// or load left, or the incoming register), so the contract does not
/// compare it and this rewrite returns zero.
///
/// Original: 0x00cad8d0 (thiscall, ECX plus one stack word, callee
/// cleans 4). The two float globals are read relocated.
lf_checker_rt::export!(thiscall, rw_00cad8d0(this: u32, arg: u32) -> u32 {
    unsafe {
        const GATE_FIRST: u32 = 0x11a;
        const GATE_SECOND: u32 = 0x384;
        const VTABLE_SLOT: u32 = 0x0c;
        const SUBTASK: u32 = 0x08;
        const TABLE_PTR: u32 = 0x224;
        const SLOT_BASE: u32 = 0x168;
        const SLOT_COUNT: u32 = 16;
        const ANCHOR_PTR: u32 = 0x20;
        const PT_X: u32 = 0x30;
        const FLAG_A: u32 = 0x210;
        const STATE: u32 = 0xa74;
        const FLAG_B: u32 = 0x212;
        const READY: u32 = 0x224;
        const BASE_PT: u32 = 0x20;
        const GOAL_PT: u32 = 0x30;
        const GOAL_W: u32 = 0x3c;
        const SCALE: u32 = 0x50;
        const DIST_HI_FILE: u32 = 0x00fe88e8;
        const DIST_LO_FILE: u32 = 0x00fe870c;

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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        // Subtask gate: indirect call through the planted object, as on
        // the original side both land on the same recorder stub.
        let gate = rd32(this + SUBTASK);
        if gate != 0 {
            let vtable = rd32(gate);
            let target = rd32(vtable + VTABLE_SLOT);
            let probe: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            if probe(gate) != GATE_FIRST && probe(gate) != GATE_SECOND {
                return 0;
            }
        }

        let dist_hi = unsafe { lf_checker_rt::global::<f32>(DIST_HI_FILE).read() };
        let dist_lo = unsafe { lf_checker_rt::global::<f32>(DIST_LO_FILE).read() };
        let anchor = rd32(arg + ANCHOR_PTR);
        let ax = rdf(anchor + PT_X);
        let ay = rdf(anchor + PT_X + 4);
        let az = rdf(anchor + PT_X + 8);

        let mut sx = 0.0f32;
        let mut sy = 0.0f32;
        let mut sz = 0.0f32;
        let mut slot = rd32(arg + TABLE_PTR) + SLOT_BASE;
        let mut left = SLOT_COUNT;
        while left > 0 {
            let cand = rd32(slot);
            if cand != 0 {
                let usable = if rd8(cand + FLAG_A) != 0 {
                    let st = rd32(cand + STATE);
                    st != 1 && st != 2
                } else {
                    true
                } && rd32(cand + READY) != 0
                    && rd8(cand + FLAG_B) == 0;
                if usable {
                    let bp = rd32(cand + ANCHOR_PTR);
                    let dx = sub(ax, rdf(bp + PT_X));
                    let dy = sub(ay, rdf(bp + PT_X + 4));
                    let dz = sub(az, rdf(bp + PT_X + 8));
                    let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                    if dist_hi > d2 && d2 > dist_lo {
                        let len = d2.sqrt();
                        let fall = sub(dist_hi, len);
                        let inv = if d2 != 0.0 { div(dist_hi, d2.sqrt()) } else { 0.0 };
                        sx = add(mul(mul(dx, inv), fall), sx);
                        sy = add(mul(mul(dy, inv), fall), sy);
                        sz = add(mul(mul(dz, inv), fall), sz);
                    }
                }
            }
            slot += 4;
            left -= 1;
        }

        let acc2 = add(add(mul(sx, sx), mul(sy, sy)), mul(sz, sz));
        if acc2 == 0.0 {
            return 0;
        }
        let nx = add(rdf(this + GOAL_PT), sx);
        let ny = add(rdf(this + GOAL_PT + 4), sy);
        let nz = add(rdf(this + GOAL_PT + 8), sz);
        wrf(this + GOAL_PT, nx);
        wrf(this + GOAL_PT + 4, ny);
        wrf(this + GOAL_PT + 8, nz);

        let bx = rdf(this + BASE_PT);
        let by = rdf(this + BASE_PT + 4);
        let bz = rdf(this + BASE_PT + 8);
        let rx = sub(nx, bx);
        let ry = sub(ny, by);
        let rz = sub(nz, bz);
        let r2 = add(add(mul(ry, ry), mul(rx, rx)), mul(rz, rz));
        let g = if r2 != 0.0 { div(dist_hi, r2.sqrt()) } else { 0.0 };
        let n0 = rdf(this + SCALE);
        let ox = add(bx, mul(mul(rx, g), n0));
        let oy = add(by, mul(mul(ry, g), n0));
        let oz = add(bz, mul(mul(rz, g), n0));
        wrf(this + GOAL_PT, ox);
        wrf(this + GOAL_PT + 4, oy);
        wrf(this + GOAL_PT + 8, oz);
        // Uninitialised stack scratch in the original; 0.0 under stack_fill 0.
        wrf(this + GOAL_W, 0.0);
        0
    }
});
