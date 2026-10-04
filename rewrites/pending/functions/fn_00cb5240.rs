// original: 0x00CB5240 subtask_factory_go_to_point (proposed)

/// Build a go-to-point subtask of the requested kind.
///
/// `this` is the complex task, `ped` the ped, `kind` the subtask id.
/// Kind 0x386 measures the heading from the ped to the target
/// (`[ped+0x20]` at +0x30/+0x34 against `this+0x20/+0x24`, callee 1,
/// float result), fetches a worker (callee 2) and builds an aim subtask
/// on it (callee 3: heading, 1.0, and the fixed 0x3ca3d70a constant).
/// Kind 0x387 fetches a worker and builds a go-to subtask on it
/// (callee 4: stored speed, target pointer, two stored floats, and the
/// two stored mode bytes). A missing worker, and any other kind, yield
/// null.
///
/// Original: 0x00CB5240 (thiscall, receiver in ECX, two stack words).
lf_checker_rt::export!(thiscall, rw_00CB5240(this: u32, ped: u32, kind: u32) -> u32 {
    unsafe {
        const AIM_AT: u32 = 1;
        const GET_WORKER: u32 = 2;
        const MAKE_AIM: u32 = 3;
        const MAKE_GO_TO: u32 = 4;
        const ALLOCATOR_GLOBAL: u32 = 0x167e2a0;
        const PED_MATRIX: u32 = 0x20;
        const TARGET: u32 = 0x20;
        const SPEED: u32 = 0x18;
        const AIM_KIND: u32 = 0x386;
        const GO_TO_KIND: u32 = 0x387;
        const AIM_BLEND: u32 = 0x3ca3d70a;
        const ONE: u32 = 0x3f800000;
        if kind == AIM_KIND {
            let matrix = (ped.wrapping_add(PED_MATRIX) as *const u32).read_unaligned();
            let mx = (matrix.wrapping_add(0x30) as *const u32).read_unaligned();
            let my = (matrix.wrapping_add(0x34) as *const u32).read_unaligned();
            let tx = (this.wrapping_add(TARGET) as *const u32).read_unaligned();
            let ty = (this.wrapping_add(TARGET + 4) as *const u32).read_unaligned();
            let heading: f32 = lf_checker_rt::callee_cdecl!(AIM_AT, f32, mx, my, tx, ty);
            let alloc = (lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL)).read_unaligned();
            let worker: u32 = lf_checker_rt::callee_thiscall!(GET_WORKER, u32, alloc);
            if worker == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(
                MAKE_AIM, u32, worker, heading.to_bits(), ONE, AIM_BLEND
            );
        }
        if kind != GO_TO_KIND {
            return 0;
        }
        let alloc = (lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL)).read_unaligned();
        let worker: u32 = lf_checker_rt::callee_thiscall!(GET_WORKER, u32, alloc);
        if worker == 0 {
            return 0;
        }
        let speed = (this.wrapping_add(SPEED) as *const u32).read_unaligned();
        let f30 = (this.wrapping_add(0x30) as *const u32).read_unaligned();
        let f34 = (this.wrapping_add(0x34) as *const u32).read_unaligned();
        let mode0 = (this.wrapping_add(0x38) as *const u8).read_unaligned() as u32;
        let mode1 = (this.wrapping_add(0x39) as *const u8).read_unaligned() as u32;
        lf_checker_rt::callee_thiscall!(
            MAKE_GO_TO, u32, worker, speed, this.wrapping_add(TARGET), f30, f34,
            mode0, mode1
        )
    }
});
