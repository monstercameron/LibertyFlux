// original: 0x00b70c60 go_to_car_door_within_radius (proposed)
/// Test whether a target point is strictly inside a radius around a source point.
///
/// `this` points to the task: dword at `+0x14` points to an inner record whose
/// dword at `+0x20` points to the source point, and float at `+0x28` is the
/// radius. `arg` points to a record whose dword at `+0x20` points to the target
/// point. Both points hold three floats at `+0x30`, `+0x34`, `+0x38`.
///
/// Returns 1 when radius*radius is strictly above the squared distance
/// (dz*dz + dy*dy) + dx*dx formed exactly as below, else 0. A NaN on either
/// side compares unordered and returns 0, matching `comiss`+`seta`.
///
/// Original: 0x00b70c60 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00b70c60(this: u32, arg: u32) -> u32 {
    unsafe {
        const INNER_OFF: u32 = 0x14;
        const RADIUS_OFF: u32 = 0x28;
        const POINT_OFF: u32 = 0x20;
        const PX: u32 = 0x30;
        const PY: u32 = 0x34;
        const PZ: u32 = 0x38;
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
        let inner = ((this + INNER_OFF) as *const u32).read_unaligned();
        let r = ((this + RADIUS_OFF) as *const f32).read_unaligned();
        let src = ((inner + POINT_OFF) as *const u32).read_unaligned();
        let targ_rec = ((arg + POINT_OFF) as *const u32).read_unaligned();
        let sx = ((src + PX) as *const f32).read_unaligned();
        let sy = ((src + PY) as *const f32).read_unaligned();
        let sz = ((src + PZ) as *const f32).read_unaligned();
        let dx = sub(sx, ((targ_rec + PX) as *const f32).read_unaligned());
        let dy = sub(sy, ((targ_rec + PY) as *const f32).read_unaligned());
        let dz = sub(sz, ((targ_rec + PZ) as *const f32).read_unaligned());
        let r2 = mul(r, r);
        let x2 = mul(dx, dx);
        let y2 = mul(dy, dy);
        let z2 = mul(dz, dz);
        let d2 = add(add(y2, x2), z2);
        (r2 > d2) as u32
    }
});

