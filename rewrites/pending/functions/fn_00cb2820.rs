// original: 0x00CB2820 CTaskComplexMoveCrossRoadAtTrafficLights::vf19

/// Cross the road, timing the crossing from the traffic-light phase.
///
/// `this` is the complex task, `ped` the ped. The light phase
/// (`[ped+0x21c]+0x12c`) decides the crossing speed: phase 2 uses the
/// plain 1.0 constant, any other phase scales the random draw (callee 1)
/// by the two rate constants and adds the base. A worker is then fetched
/// (callee 2); a missing worker crashes on a null access, exactly as the
/// original does. Otherwise a cross subtask is built on it (callee 3, ten
/// words: crossing speed, point pointer, 1.0, the 3.0 constant, -1, 1, 0,
/// 0, 0, 1), the crossing flag (bit 13) is set on the result and its
/// blend slot is fixed at 2.0.
///
/// Original: 0x00CB2820 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB2820(this: u32, ped: u32) -> u32 {
    unsafe {
        const RANDOM_DRAW: u32 = 1;
        const GET_WORKER: u32 = 2;
        const MAKE_CROSS: u32 = 3;
        const ALLOCATOR_GLOBAL: u32 = 0x167e2a0;
        const RATE_A_GLOBAL: u32 = 0xfe8684;
        const RATE_B_GLOBAL: u32 = 0xfe8830;
        const BASE_GLOBAL: u32 = 0xfe88e8;
        const FIXED_GLOBAL: u32 = 0xee1eb4;
        const PED_LIGHTS: u32 = 0x21c;
        const PHASE: u32 = 0x12c;
        const GREEN_PHASE: u32 = 2;
        const POINT: u32 = 0x20;
        const CROSS_FLAG: u32 = 0x2000;
        const BLEND_TWO: u32 = 0x40000000;
        let lights = (ped.wrapping_add(PED_LIGHTS) as *const u32).read_unaligned();
        let phase = (lights.wrapping_add(PHASE) as *const u32).read_unaligned();
        let base = f32::from_bits(
            (lf_checker_rt::global::<u32>(BASE_GLOBAL)).read_unaligned(),
        );
        let speed = if phase == GREEN_PHASE {
            base
        } else {
            let draw: u32 = lf_checker_rt::callee_thiscall!(RANDOM_DRAW, u32, this);
            let rate_a = f32::from_bits(
                (lf_checker_rt::global::<u32>(RATE_A_GLOBAL)).read_unaligned(),
            );
            let rate_b = f32::from_bits(
                (lf_checker_rt::global::<u32>(RATE_B_GLOBAL)).read_unaligned(),
            );
            let scaled = core::hint::black_box(draw as i32 as f32);
            let step_a = core::hint::black_box(scaled) * core::hint::black_box(rate_a);
            let step_b = core::hint::black_box(step_a) * core::hint::black_box(rate_b);
            core::hint::black_box(step_b) + core::hint::black_box(base)
        };
        let alloc = (lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL)).read_unaligned();
        let worker: u32 = lf_checker_rt::callee_thiscall!(GET_WORKER, u32, alloc);
        let fixed = (lf_checker_rt::global::<u32>(FIXED_GLOBAL)).read_unaligned();
        let one = 0x3f800000u32;
        let task = if worker == 0 {
            0u32
        } else {
            lf_checker_rt::callee_thiscall!(
                MAKE_CROSS, u32, worker, speed.to_bits(), this.wrapping_add(POINT),
                one, fixed, 0xffff_ffff, 1, 0, 0, 0, 1
            )
        };
        // The original flags the result unconditionally, faulting on the
        // null worker path; the opaque address keeps a real fault.
        let slot = core::hint::black_box(task).wrapping_add(0xd8);
        let flags = (slot as *const u32).read_unaligned();
        (slot as *mut u32).write_unaligned(flags | CROSS_FLAG);
        (core::hint::black_box(task).wrapping_add(0x5c) as *mut u32)
            .write_unaligned(BLEND_TWO);
        task
    }
});
