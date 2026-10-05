// original: 0x00DA7B10 CTaskComplexFleeAndDive::vf19

/// Pick the dive-or-flee request code from the distance to the threat, then
/// tail-jump to the shared dispatcher.
///
/// `this` is the flee-and-dive task, `ped` the ped it runs on. The ped's
/// `+0x20` pointer leads to a position vector at `+0x30/+0x34/+0x38`; the
/// task holds its own anchor at `+0x20/+0x24/+0x28` and a radius at `+0x38`.
/// The function measures the distance between the two points (squared
/// differences summed dx-first, then square-rooted) and compares the radius
/// against it: radius strictly above the distance selects `0x3AE`, anything
/// else (including an unordered NaN comparison) selects `0x1F6`. The code
/// overwrites the incoming stack argument and control tail-jumps to the
/// dispatcher, which is modelled as a call that forwards `this` and the
/// code and returns its answer.
///
/// Only the entry at this address is covered: the inventory range holds
/// several more entries past the padding that follows the jump.
///
/// Original: 0x00DA7B10 (thiscall, one stack word, tail jump, returns eax).
lf_checker_rt::export!(thiscall, rw_00DA7B10(this: u32, ped: u32) -> u32 {
    unsafe {
        const PED_POS_SRC: u32 = 0x20;
        const VEC_X: u32 = 0x30;
        const VEC_Y: u32 = 0x34;
        const VEC_Z: u32 = 0x38;
        const TASK_X: u32 = 0x20;
        const TASK_Y: u32 = 0x24;
        const TASK_Z: u32 = 0x28;
        const TASK_RADIUS: u32 = 0x38;
        const CODE_NEAR: u32 = 0x3AE;
        const CODE_FAR: u32 = 0x1F6;

        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
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

        let vec: u32 = (ped.wrapping_add(PED_POS_SRC) as *const u32).read_unaligned();
        let dx: f32 = sub(rdf(vec.wrapping_add(VEC_X)), rdf(this.wrapping_add(TASK_X)));
        let dy: f32 = sub(rdf(vec.wrapping_add(VEC_Y)), rdf(this.wrapping_add(TASK_Y)));
        let dz: f32 = sub(rdf(vec.wrapping_add(VEC_Z)), rdf(this.wrapping_add(TASK_Z)));
        let dist2: f32 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
        // One square root exactly as the single-precision instruction forms it.
        let dist: f32 = unsafe {
            core::arch::x86::_mm_cvtss_f32(core::arch::x86::_mm_sqrt_ss(
                core::arch::x86::_mm_set_ss(core::hint::black_box(dist2)),
            ))
        };
        let radius: f32 = rdf(this.wrapping_add(TASK_RADIUS));
        let code: u32 = if radius > dist { CODE_NEAR } else { CODE_FAR };
        lf_checker_rt::callee_thiscall!(1, u32, this, code)
    }
});
