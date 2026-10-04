// original: 0x00CB3130 CTaskComplexTurnToFaceEntityOrCoord::vf19

/// Turn the ped toward its target entity or coordinate.
///
/// `this` is the complex task, `ped` the ped. When entity-facing is on
/// (`this+0x18` non-zero) a null entity (`this+0x14`) yields null at once.
/// Otherwise the facing heading is computed (callee 1, float result) and
/// a worker is fetched from the global allocator (callee 2): with one, a
/// turn subtask is built (callee 3) from the heading and the stored pitch
/// pair (`this+0x30`, `this+0x34`); without one the turn step is skipped.
/// A second worker fetch (callee 4, same callee as 2 but an independent
/// scripted answer so every worker combination is exercised) guards the
/// final face subtask (callee 5); a missing worker yields null.
///
/// Original: 0x00CB3130 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB3130(this: u32, ped: u32) -> u32 {
    unsafe {
        const FACE_HEADING: u32 = 1;
        const GET_WORKER_A: u32 = 2;
        const MAKE_TURN: u32 = 3;
        const GET_WORKER_B: u32 = 4;
        const MAKE_FACE: u32 = 5;
        const ALLOCATOR_GLOBAL: u32 = 0x167e2a0;
        const FACE_ENTITY: u32 = 0x18;
        const ENTITY: u32 = 0x14;
        const PITCH: u32 = 0x30;
        if (this.wrapping_add(FACE_ENTITY) as *const u8).read_unaligned() != 0
            && (this.wrapping_add(ENTITY) as *const u32).read_unaligned() == 0
        {
            return 0;
        }
        let heading: f32 = lf_checker_rt::callee_thiscall!(FACE_HEADING, f32, this, ped);
        let alloc = (lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL)).read_unaligned();
        let worker: u32 = lf_checker_rt::callee_thiscall!(GET_WORKER_A, u32, alloc);
        let mut turn = 0u32;
        if worker != 0 {
            let pitch0 = (this.wrapping_add(PITCH) as *const u32).read_unaligned();
            let pitch1 = (this.wrapping_add(PITCH + 4) as *const u32).read_unaligned();
            turn = lf_checker_rt::callee_thiscall!(
                MAKE_TURN, u32, worker, heading.to_bits(), pitch0, pitch1
            );
        }
        let alloc2 = (lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL)).read_unaligned();
        let worker2: u32 = lf_checker_rt::callee_thiscall!(GET_WORKER_B, u32, alloc2);
        if worker2 == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(MAKE_FACE, u32, worker2, turn, 0, 0, 0)
    }
});
