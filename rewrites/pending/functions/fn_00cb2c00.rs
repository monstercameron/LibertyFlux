// original: 0x00CB2C00 CTaskComplexMoveGetToPointContinuous::vf19

/// Chase the moving target point, or report arrival when inside the radius.
///
/// `this` is the complex task, `ped` the ped. The squared distance from
/// the target (`this+0x30/+0x34`) to the ped (`[ped+0x20]` at +0x30/+0x34)
/// is compared against the squared arrival radius (`this+0x20` squared;
/// a NaN comparison counts as arrived, matching the original's
/// jump-if-below-or-equal). When still outside, a worker is fetched
/// (callee 1) and a go-to subtask is built on it (callee 2, ten words:
/// stored speed, target pointer, two stored floats, -1, 1, 0, 0, 0, 1);
/// a missing worker yields null. When inside, a worker is fetched and
/// initialised (callee 3, flag 1) as the arrived marker (vtable pair and
/// a zeroed slot) and returned; a missing worker yields null.
///
/// Original: 0x00CB2C00 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB2C00(this: u32, ped: u32) -> u32 {
    unsafe {
        const GET_WORKER: u32 = 1;
        const MAKE_GO_TO: u32 = 2;
        const INIT_MARKER: u32 = 3;
        const ALLOCATOR_GLOBAL: u32 = 0x167e2a0;
        const RADIUS: u32 = 0x20;
        const TARGET: u32 = 0x30;
        const SPEED: u32 = 0x18;
        const PED_MATRIX: u32 = 0x20;
        const MARKER_VT0: u32 = 0xe98794;
        const MARKER_VT1: u32 = 0xe987e8;
        // File VAs: the worker relocates the image, so derive the address.
        let vt0 = lf_checker_rt::relocated(MARKER_VT0);
        let vt1 = lf_checker_rt::relocated(MARKER_VT1);
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
        let arg = (ped.wrapping_add(PED_MATRIX) as *const u32).read_unaligned();
        let dx = sub(rdf(arg.wrapping_add(0x30)), rdf(this.wrapping_add(TARGET)));
        let dy = sub(
            rdf(arg.wrapping_add(0x34)),
            rdf(this.wrapping_add(TARGET + 4)),
        );
        let radius = rdf(this.wrapping_add(RADIUS));
        // jbe after comiss(d2, r2): taken unless d2 is strictly greater.
        if !(add(mul(dx, dx), mul(dy, dy)) > mul(radius, radius)) {
            let alloc = (lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL)).read_unaligned();
            let marker: u32 = lf_checker_rt::callee_thiscall!(GET_WORKER, u32, alloc);
            if marker == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(INIT_MARKER, u32, marker, 1);
            (marker as *mut u32).write_unaligned(vt0);
            (marker.wrapping_add(0x14) as *mut u32).write_unaligned(vt1);
            (marker.wrapping_add(0x20) as *mut u32).write_unaligned(0);
            return marker;
        }
        let alloc = (lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL)).read_unaligned();
        let worker: u32 = lf_checker_rt::callee_thiscall!(GET_WORKER, u32, alloc);
        if worker == 0 {
            return 0;
        }
        let speed = (this.wrapping_add(SPEED) as *const u32).read_unaligned();
        let f24 = (this.wrapping_add(0x24) as *const u32).read_unaligned();
        let f28 = (this.wrapping_add(0x28) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            MAKE_GO_TO, u32, worker, speed, this.wrapping_add(TARGET), f24, f28,
            0xffff_ffff, 1, 0, 0, 0, 1
        )
    }
});
