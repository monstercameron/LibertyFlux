// original: 0x00D09CC0 CTaskComplexCombatClosestTargetInArea::vf19

/// Scan the configured area, keep the best eligible combat target, and build
/// a combat task for it.
///
/// The scanner center and radius come from `self+0x20..0x30`; when the flag at
/// `self+0x34` is set, the center is replaced with the incoming ped's position
/// vector at `ped+0x20+0x30`. Each scanner result is filtered by state bytes
/// `+0x210` and `+0x211`, mode word `+0xa74`, and target pointer `+0x224`, then
/// checked through ped-intelligence helpers. The distance score is computed
/// from center-minus-target deltas in the original Y, X, Z operand order, with
/// each square and sum kept in that order. The first eligible target is kept;
/// later targets replace it only when their score is greater. A task is
/// allocated and constructed for the selected entity, and the scanner is
/// destroyed on every return path.
///
/// Scanner construction, iteration, ped filters, pool lookup, task creation,
/// and scanner cleanup are intercepted by the proof contract. The scanner
/// object and center pointer are stack locals; their addresses are omitted
/// from call comparison while snapshots compare their contents.
lf_checker_rt::export!(thiscall, rw_00d09cc0(owner: u32, ped: u32) -> u32 {
    unsafe {
        const PED_OVERRIDE_CENTER: u32 = 0x34;
        const PED_SCANNER_CONTEXT: u32 = 0x20;
        const OWNER_CENTER_X: u32 = 0x20;
        const OWNER_CENTER_Y: u32 = 0x24;
        const OWNER_CENTER_Z: u32 = 0x28;
        const OWNER_RADIUS: u32 = 0x30;
        const OWNER_TASK_RADIUS: u32 = 0x38;
        const PED_POSITION: u32 = 0x30;
        const ENTITY_STATE: u32 = 0x210;
        const ENTITY_FLAG: u32 = 0x211;
        const ENTITY_INTELLIGENCE: u32 = 0x224;
        const ENTITY_MODE: u32 = 0xA74;
        const INTELLIGENCE: u32 = 0x224;
        const TASK_POOL_GLOBAL: u32 = 0x0167_E2A0;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(address) })
        }

        #[inline(always)]
        fn sub_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }

        #[inline(always)]
        fn mul_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        #[inline(always)]
        fn add_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        #[inline(always)]
        fn sqrt_f32(value: f32) -> f32 {
            use core::arch::x86::{_mm_cvtss_f32, _mm_set_ss, _mm_sqrt_ss};
            unsafe { _mm_cvtss_f32(_mm_sqrt_ss(_mm_set_ss(value))) }
        }

        let mut scanner = [0u32; 6];
        let mut center = [0u32; 4];
        center[0] = unsafe { read_u32(owner.wrapping_add(OWNER_CENTER_X)) };
        center[1] = unsafe { read_u32(owner.wrapping_add(OWNER_CENTER_Y)) };
        center[2] = unsafe { read_u32(owner.wrapping_add(OWNER_CENTER_Z)) };
        center[3] = 0;

        if unsafe { read_u8(owner.wrapping_add(PED_OVERRIDE_CENTER)) } != 0 {
            let position = unsafe { read_u32(ped.wrapping_add(PED_SCANNER_CONTEXT)) };
            center[0] = unsafe { read_u32(position.wrapping_add(PED_POSITION)) };
            center[1] = unsafe { read_u32(position.wrapping_add(PED_POSITION + 4)) };
            center[2] = unsafe { read_u32(position.wrapping_add(PED_POSITION + 8)) };
            center[3] = unsafe { read_u32(position.wrapping_add(PED_POSITION + 12)) };
        }

        let scanner_ptr = scanner.as_mut_ptr() as u32;
        let center_ptr = center.as_mut_ptr() as u32;
        let radius = unsafe { read_u32(owner.wrapping_add(OWNER_RADIUS)) };
        let _ = unsafe {
            lf_checker_rt::callee_thiscall!(1, u32, scanner_ptr, 1, ped, center_ptr, radius)
        };

        let mut best_entity = 0u32;
        let mut best_distance = 0.0f32;
        loop {
            let entity = unsafe { lf_checker_rt::callee_thiscall!(2, u32, scanner_ptr) };
            if entity == 0 {
                break;
            }

            let state_allows_scan = unsafe { read_u8(entity.wrapping_add(ENTITY_STATE)) } == 0;
            let mode = unsafe { read_u32(entity.wrapping_add(ENTITY_MODE)) };
            let mode_allows_scan = mode != 1 && mode != 2;
            let target = unsafe { read_u32(entity.wrapping_add(ENTITY_INTELLIGENCE)) };
            if state_allows_scan && mode_allows_scan && target != 0 {
                let required_flag = unsafe { read_u8(entity.wrapping_add(ENTITY_FLAG)) } != 0;
                let flag_check_passes = !required_flag
                    || unsafe { lf_checker_rt::callee_thiscall!(3, u32, ped) } & 0xFF != 0;
                let intelligence = unsafe { read_u32(ped.wrapping_add(INTELLIGENCE)) };
                let target_check_passes = flag_check_passes
                    && unsafe {
                        lf_checker_rt::callee_thiscall!(4, u32, intelligence, entity, 1)
                    } & 0xFF
                        != 0;

                if target_check_passes {
                    let _ = unsafe {
                        lf_checker_rt::callee_thiscall!(5, u32, intelligence, entity, 1)
                    };
                    let position = unsafe { read_u32(entity.wrapping_add(PED_SCANNER_CONTEXT)) };
                    let x_delta = sub_f32(
                        f32::from_bits(center[0]),
                        unsafe { read_f32(position.wrapping_add(PED_POSITION)) },
                    );
                    let y_delta = sub_f32(
                        f32::from_bits(center[1]),
                        unsafe { read_f32(position.wrapping_add(PED_POSITION + 4)) },
                    );
                    let z_delta = sub_f32(
                        f32::from_bits(center[2]),
                        unsafe { read_f32(position.wrapping_add(PED_POSITION + 8)) },
                    );
                    let y_squared = mul_f32(y_delta, y_delta);
                    let x_squared = mul_f32(x_delta, x_delta);
                    let z_squared = mul_f32(z_delta, z_delta);
                    let xy_squared = add_f32(y_squared, x_squared);
                    let distance = sqrt_f32(add_f32(xy_squared, z_squared));
                    if best_entity == 0 || distance > best_distance {
                        best_entity = entity;
                        best_distance = distance;
                    }
                }
            }
        }

        if best_entity == 0 {
            let _ = unsafe { lf_checker_rt::callee_thiscall!(8, u32, scanner_ptr) };
            return 0;
        }

        let singleton_slot = lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL);
        let singleton = unsafe { singleton_slot.read_unaligned() };
        let pool = unsafe { lf_checker_rt::callee_thiscall!(6, u32, singleton) };
        if pool == 0 {
            let _ = unsafe { lf_checker_rt::callee_thiscall!(8, u32, scanner_ptr) };
            return 0;
        }

        let task_radius = unsafe { read_u32(owner.wrapping_add(OWNER_TASK_RADIUS)) };
        let task = unsafe {
            lf_checker_rt::callee_thiscall!(7, u32, pool, best_entity, task_radius)
        };
        let _ = unsafe { lf_checker_rt::callee_thiscall!(8, u32, scanner_ptr) };
        task
    }
});
