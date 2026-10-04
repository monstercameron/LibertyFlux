// original: 0x00CB3600 CTaskComplexMoveAroundCoverPoints::vf18

/// Re-aim the ped at a new cover point once the old one is reached.
///
/// `this` is the complex task, `ped` the ped. When cover switching is
/// allowed (`this+0x54` zero) and the ped is already within the switch
/// radius (squared distance below the 0.25 constant, a NaN distance
/// counts as outside) of the current point (`this+0x20/+0x24` against
/// `[ped+0x20]` at +0x30/+0x34), and the cover slot (`this+0x58`) is not
/// already slot 2, the cover refresh (callee 1) and the cover search
/// (callee 2: ped, point slot, refresh result, slot pointer, and 0;
/// caller cleans up) run. When the search succeeds, the move setup
/// (callee 3) runs and the distance is measured again (the callees move
/// the task's point in the real game; the checker stubs model that with
/// scripted writes): only when it is now outside the radius is a worker
/// fetched (callee 4) and a go-to subtask built on it (callee 5) from the
/// stored speed (`this+0x18`, by bits) and the new point with the fixed
/// blend constant 0x3d4ccccd. Every early exit yields null.
///
/// Original: 0x00CB3600 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB3600(this: u32, ped: u32) -> u32 {
    unsafe {
        const REFRESH_COVER: u32 = 1;
        const SEARCH_COVER: u32 = 2;
        const SETUP_MOVE: u32 = 3;
        const GET_WORKER: u32 = 4;
        const MAKE_GO_TO: u32 = 5;
        const ALLOCATOR_GLOBAL: u32 = 0x167e2a0;
        const RADIUS2_GLOBAL: u32 = 0xfe87e4;
        const CAN_SWITCH: u32 = 0x54;
        const COVER_SLOT: u32 = 0x58;
        const HELD_SLOT: u32 = 2;
        const POINT: u32 = 0x20;
        const SPEED: u32 = 0x18;
        const PED_MATRIX: u32 = 0x20;
        const BLEND: u32 = 0x3d4ccccd;
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        if (this.wrapping_add(CAN_SWITCH) as *const u8).read_unaligned() != 0 {
            return 0;
        }
        let limit = f32::from_bits(
            (lf_checker_rt::global::<u32>(RADIUS2_GLOBAL)).read_unaligned(),
        );
        let matrix = (ped.wrapping_add(PED_MATRIX) as *const u32).read_unaligned();
        let dx = sub(rdf(this.wrapping_add(POINT)), rdf(matrix.wrapping_add(0x30)));
        let dy = sub(
            rdf(this.wrapping_add(POINT + 4)),
            rdf(matrix.wrapping_add(0x34)),
        );
        // jbe after comiss(const, d2): taken unless const is strictly greater.
        if !(limit > add(mul(dx, dx), mul(dy, dy))) {
            return 0;
        }
        if (this.wrapping_add(COVER_SLOT) as *const u32).read_unaligned() == HELD_SLOT {
            return 0;
        }
        let found: u32 = lf_checker_rt::callee_thiscall!(REFRESH_COVER, u32, this, ped);
        let ok: u32 = lf_checker_rt::callee_cdecl!(
            SEARCH_COVER, u32, ped, this.wrapping_add(0x30), found,
            this.wrapping_add(COVER_SLOT), 0
        );
        if ok as u8 == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(SETUP_MOVE, u32, this, ped);
        let ex = sub(rdf(this.wrapping_add(POINT)), rdf(matrix.wrapping_add(0x30)));
        let ey = sub(
            rdf(this.wrapping_add(POINT + 4)),
            rdf(matrix.wrapping_add(0x34)),
        );
        // jbe after comiss(d2, const): taken unless d2 is strictly greater.
        if !(add(mul(ex, ex), mul(ey, ey)) > limit) {
            return 0;
        }
        let alloc = (lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL)).read_unaligned();
        let worker: u32 = lf_checker_rt::callee_thiscall!(GET_WORKER, u32, alloc);
        if worker == 0 {
            return 0;
        }
        let speed = (this.wrapping_add(SPEED) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            MAKE_GO_TO, u32, worker, speed, this.wrapping_add(POINT), BLEND, 0, 0
        )
    }
});
