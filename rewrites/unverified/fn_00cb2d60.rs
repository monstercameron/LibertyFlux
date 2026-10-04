// original: 0x00cb2d60 CTaskComplexMoveGoToPointRelativeToEntityAndStandStill::vf19

/// Refresh the target point from the entity's matrix and request the move.
///
/// `this` is the complex task (the pedestrian argument is unused). Reads
/// the entity at `+0x40`, a countdown at `+0x48`, the local offset at
/// `+0x20`, and two floats at `+0x18`/`+0x44`. Returns 0 when there is no
/// entity or no move system.
///
/// Flow: with no entity, return 0. With a positive countdown, snapshot a
/// global tick into `+0x4c`, the countdown into `+0x50`, and set the flag at
/// `+0x54`. When the entity's matrix slot (`+0x20`) is empty, run the two
/// entity callees (ids 1, 2) that fill it. Transform the local offset by the
/// 3x4 matrix (columns at `+0x00`, `+0x10`, `+0x20`, translation at `+0x30`)
/// into `+0x30`/`+0x34`/`+0x38`; the `+0x3c` lane is taken from an
/// uninitialized stack slot in the original, which the contract pins to 0
/// via `stack_fill` (see below). Then ask the move singleton (callee 3);
/// on null return 0, else request the move through callee 4 with
/// (`+0x18` bits, `+0x30` pointer, `+0x44` bits, 2.0, 0, 0) and return its
/// answer.
///
/// Uninitialized slot: the original loads `[esp+0x1c]` from its own frame
/// without ever storing there (only completed callee calls ran between),
/// so the value is whatever the callees left behind. No faithful Rust can
/// reproduce that; the rewrite stores the contract's defined fill (0).
/// Float operation order matches the original exactly.
///
/// Original: 0x00cb2d60 (thiscall, one ignored stack word).
lf_checker_rt::export!(thiscall, rw_00cb2d60(this: u32, _ped: u32) -> u32 {
    unsafe {
        const ENTITY: u32 = 0x40;
        const COUNTDOWN: u32 = 0x48;
        const TICK_SNAP: u32 = 0x4c;
        const COUNT_SNAP: u32 = 0x50;
        const DIRTY: u32 = 0x54;
        const LOCAL: u32 = 0x20;
        const TARGET: u32 = 0x30;
        const ARG_A: u32 = 0x18;
        const ARG_B: u32 = 0x44;
        const MTX_SLOT: u32 = 0x20;
        const CTX_OFF: u32 = 0x10;
        const TICK_FILE_VA: u32 = 0x011735b4;
        const SINGLETON_FILE_VA: u32 = 0x0167e2a0;
        const UNINIT_FILL: u32 = 0;
        const FIX_CALLEE: u32 = 1;
        const PLACE_CALLEE: u32 = 2;
        const SINGLETON_CALLEE: u32 = 3;
        const REQUEST_CALLEE: u32 = 4;

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

        let ent = rd32(this + ENTITY);
        if ent == 0 {
            return 0;
        }
        let n = rd32(this + COUNTDOWN) as i32;
        if n > 0 {
            wr32(this + TICK_SNAP, rd32(lf_checker_rt::relocated(TICK_FILE_VA)));
            wr32(this + COUNT_SNAP, n as u32);
            ((this + DIRTY) as *mut u8).write(1);
        }
        if rd32(ent + MTX_SLOT) == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(FIX_CALLEE, u32, ent);
            let m = rd32(ent + MTX_SLOT);
            let _: u32 = lf_checker_rt::callee_thiscall!(PLACE_CALLEE, u32, ent + CTX_OFF, m);
        }
        let m = rd32(ent + MTX_SLOT);
        let v0 = rdf(this + LOCAL);
        let v1 = rdf(this + LOCAL + 4);
        let v2 = rdf(this + LOCAL + 8);
        let ox = add(
            add(add(mul(rdf(m + 0x10), v1), mul(rdf(m), v0)), mul(rdf(m + 0x20), v2)),
            rdf(m + 0x30),
        );
        let oy = add(
            add(add(mul(rdf(m + 0x14), v1), mul(rdf(m + 4), v0)), mul(rdf(m + 0x24), v2)),
            rdf(m + 0x34),
        );
        let oz = add(
            add(add(mul(rdf(m + 0x18), v1), mul(rdf(m + 8), v0)), mul(rdf(m + 0x28), v2)),
            rdf(m + 0x38),
        );
        wrf(this + TARGET, ox);
        wrf(this + TARGET + 4, oy);
        wr32(this + TARGET + 12, UNINIT_FILL);
        wrf(this + TARGET + 8, oz);
        let sys: u32 = lf_checker_rt::callee_thiscall!(
            SINGLETON_CALLEE,
            u32,
            rd32(lf_checker_rt::relocated(SINGLETON_FILE_VA))
        );
        if sys == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(
            REQUEST_CALLEE,
            u32,
            sys,
            rd32(this + ARG_A),
            this + TARGET,
            rd32(this + ARG_B),
            0x40000000u32,
            0u32,
            0u32
        )
    }
});
