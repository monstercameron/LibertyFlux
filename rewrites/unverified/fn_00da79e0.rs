// original: 0x00DA79E0 CTaskComplexEscapeBlast::vf19

/// Decide the escaping ped's next flee task from its distance to the blast.
///
/// `this` is the escape-blast task object, `ped` the ped it runs on. The
/// task stores a target point at `+0x20/+0x24/+0x28`, a radius at `+0x30`,
/// a flag byte at `+0x38` and a selector dword at `+0x14`. The ped stores a
/// position-source pointer at `+0x20` whose `+0x30/+0x34/+0x38` floats are
/// the blast centre.
///
/// The function first runs two setup callees (a 10-argument block call with
/// ecx at `ped+0x570`, then a 2-argument call with ecx at `ped`), then
/// compares the squared distance from the target point to the blast centre
/// against the squared radius. When the blast is farther than the radius
/// and the flag byte is clear it asks the blast manager (global dword) for
/// a dispatcher and creates a dive task through it; when the blast is
/// inside the radius (or the distance is unordered, NaN) it asks the same
/// manager and creates either a directional flee task (selector non-zero,
/// passed along) or a plain flee task (selector zero, ped passed along).
/// Any null answer, or the far-but-flagged case, returns 0.
///
/// Float order is the original's: dy squared plus dx squared, plus dz
/// squared, against radius squared; the branch is "not above", so NaN
/// takes the inside-radius path. The constant pushed to the first callee is
/// a relocated image address; the constant pushed to the dive-task callee
/// is a plain literal.
///
/// Original: 0x00DA79E0 (thiscall, one stack word, returns eax).
lf_checker_rt::export!(thiscall, rw_00DA79E0(this: u32, ped: u32) -> u32 {
    unsafe {
        const PED_POS_SRC: u32 = 0x20;
        const PED_BLOCK_OFF: u32 = 0x570;
        const TASK_TARGET_X: u32 = 0x20;
        const TASK_TARGET_Y: u32 = 0x24;
        const TASK_TARGET_Z: u32 = 0x28;
        const TASK_RADIUS: u32 = 0x30;
        const TASK_FLAG: u32 = 0x38;
        const TASK_SELECTOR: u32 = 0x14;
        const VEC_X: u32 = 0x30;
        const VEC_Y: u32 = 0x34;
        const VEC_Z: u32 = 0x38;
        const BLAST_MANAGER: u32 = 0x0167E2A0;
        const SETUP_BLOCK: u32 = 0x00EEF9E4;
        const DIVE_KIND: u32 = 0x0098967F;
        const ONE_BITS: u32 = 0x3F800000;
        const FLEE_TIME: u32 = 0xF4240;
        const FLEE_RANGE: u32 = 0x3E8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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

        // Two setup calls; their answers are ignored.
        let block: u32 = lf_checker_rt::relocated(SETUP_BLOCK);
        let _ = lf_checker_rt::callee_thiscall!(
            1, u32, ped.wrapping_add(PED_BLOCK_OFF),
            block, 0, 0, 0, 0xFFFF_FFFF, 0, 0, ONE_BITS, 0, 0
        );
        let _ = lf_checker_rt::callee_thiscall!(2, u32, ped, 0, 0xFFFF_FFFF);

        // Squared distance from the target point to the blast centre.
        let centre: u32 = rd32(ped.wrapping_add(PED_POS_SRC));
        let dx: f32 = sub(rdf(this.wrapping_add(TASK_TARGET_X)), rdf(centre.wrapping_add(VEC_X)));
        let dy: f32 = sub(rdf(this.wrapping_add(TASK_TARGET_Y)), rdf(centre.wrapping_add(VEC_Y)));
        let dz: f32 = sub(rdf(this.wrapping_add(TASK_TARGET_Z)), rdf(centre.wrapping_add(VEC_Z)));
        let radius: f32 = rdf(this.wrapping_add(TASK_RADIUS));
        let dist2: f32 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        let rad2: f32 = mul(radius, radius);

        if dist2 > rad2 {
            // Blast farther than the radius.
            if (this.wrapping_add(TASK_FLAG) as *const u8).read() != 0 {
                return 0;
            }
            let manager: u32 = lf_checker_rt::global::<u32>(BLAST_MANAGER).read();
            let dispatch: u32 = lf_checker_rt::callee_thiscall!(3, u32, manager);
            if dispatch == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(4, u32, dispatch, 0, DIVE_KIND, 0xFFFF_FFFF)
        } else {
            // Blast inside the radius (or unordered distance).
            let manager: u32 = lf_checker_rt::global::<u32>(BLAST_MANAGER).read();
            let selector: u32 = rd32(this.wrapping_add(TASK_SELECTOR));
            let dispatch: u32 = lf_checker_rt::callee_thiscall!(3, u32, manager);
            if dispatch == 0 {
                return 0;
            }
            let reach: u32 = rd32(this.wrapping_add(TASK_RADIUS));
            if selector != 0 {
                lf_checker_rt::callee_thiscall!(
                    5, u32, dispatch, selector, 0, reach, FLEE_TIME, FLEE_RANGE, ONE_BITS, 0
                )
            } else {
                lf_checker_rt::callee_thiscall!(6, u32, dispatch, ped, 0, reach, FLEE_TIME, 0)
            }
        }
    }
});
