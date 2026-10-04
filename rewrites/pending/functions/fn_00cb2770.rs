// original: 0x00CB2770 CTaskComplexMoveAroundCoverPoints::vf19

/// Drive the ped to the next cover point via the task allocator.
///
/// `this` is the complex task, `ped` the ped. The cover-point refresh
/// (callee 1) and the ped's move setup (callee 2, flag 1) run first; then
/// a worker is fetched from the global allocator (callee 3) and, when one
/// is available, a go-to-point subtask is built on it (callee 4) from the
/// task's stored speed (`this+0x18`, passed by bits) and target point
/// (`this+0x20`) with the fixed blend constant 0x3d4ccccd. A missing
/// worker yields null.
///
/// Original: 0x00CB2770 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB2770(this: u32, ped: u32) -> u32 {
    unsafe {
        const REFRESH_COVER: u32 = 1;
        const SETUP_MOVE: u32 = 2;
        const GET_WORKER: u32 = 3;
        const MAKE_GO_TO: u32 = 4;
        const ALLOCATOR_GLOBAL: u32 = 0x167e2a0;
        const SPEED: u32 = 0x18;
        const TARGET: u32 = 0x20;
        const BLEND: u32 = 0x3d4ccccd;
        lf_checker_rt::callee_thiscall!(REFRESH_COVER, u32, this, ped);
        lf_checker_rt::callee_thiscall!(SETUP_MOVE, u32, ped, 1);
        let alloc = (lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL)).read_unaligned();
        let worker: u32 = lf_checker_rt::callee_thiscall!(GET_WORKER, u32, alloc);
        if worker == 0 {
            return 0;
        }
        let speed = (this.wrapping_add(SPEED) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            MAKE_GO_TO, u32, worker, speed, this.wrapping_add(TARGET), BLEND, 0, 0
        )
    }
});
