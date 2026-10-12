// original: 0x00D09F00 CTaskComplexGoIntoCover::vf19

/// Update the cover task's movement target, then create the next task for its ped.
///
/// `owner` stores the destination at `+0x20`, a movement radius at `+0x30`, a
/// speed-like value at `+0x34`, and a route flag at `+0x38`. `ped` is the
/// current ped. When the route flag is set, the method asks the route helpers
/// for a nearby point, updates the destination from the returned direction,
/// and registers the request. It then either allocates a player-on-foot task
/// for a ped with the special state byte set, or constructs a combat-fire task
/// around the stored destination. A failed route query or empty task pool
/// returns null; a null player-task constructor result faults when its state
/// byte is set, matching the original.
lf_checker_rt::export!(thiscall, rw_00d09f00(owner: u32, ped: u32) -> u32 {
    unsafe {
        const OWNER_DESTINATION: u32 = 0x20;
        const OWNER_RADIUS: u32 = 0x30;
        const OWNER_SPEED: u32 = 0x34;
        const OWNER_ROUTE_ENABLED: u32 = 0x38;
        const PED_CONTEXT: u32 = 0x20;
        const PED_ROUTE_STATE: u32 = 0x219;
        const PED_TASK_REGISTRY: u32 = 0xD68;
        const PED_POSITION: u32 = 0x30;
        const TASK_POOL_GLOBAL: u32 = 0x0167_E2A0;
        const ROUTE_SCALE_GLOBAL: u32 = 0x00FE_8AD8;
        const TASK_ACTIVE: u32 = 0x50;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(address) })
        }

        #[inline(always)]
        fn mul_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        #[inline(always)]
        fn add_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        let route_enabled = unsafe { read_u8(owner.wrapping_add(OWNER_ROUTE_ENABLED)) } != 0;
        if route_enabled {
            let _ = unsafe { lf_checker_rt::callee_thiscall!(1, u32, ped) };
            let destination = owner.wrapping_add(OWNER_DESTINATION);
            let nearest = unsafe {
                lf_checker_rt::callee_cdecl!(2, u32, destination, 0, 2, 0, 0, 1)
            };
            if nearest == 0 {
                return 0;
            }

            let _ = unsafe { lf_checker_rt::callee_thiscall!(3, u32, ped, nearest) };
            let registry = unsafe { read_u32(ped.wrapping_add(PED_TASK_REGISTRY)) };
            let route_position = unsafe {
                lf_checker_rt::callee_thiscall!(
                    4,
                    u32,
                    registry,
                    (ped.wrapping_add(PED_POSITION + 0x10))
                )
            };
            let route_scale_slot = lf_checker_rt::global::<u32>(ROUTE_SCALE_GLOBAL);
            let route_scale = f32::from_bits(unsafe { route_scale_slot.read_unaligned() });

            let x = add_f32(
                mul_f32(unsafe { read_f32(route_position) }, route_scale),
                unsafe { read_f32(destination) },
            );
            let y = add_f32(
                mul_f32(unsafe { read_f32(route_position.wrapping_add(4)) }, route_scale),
                unsafe { read_f32(destination.wrapping_add(4)) },
            );
            let z = add_f32(
                mul_f32(unsafe { read_f32(route_position.wrapping_add(8)) }, route_scale),
                unsafe { read_f32(destination.wrapping_add(8)) },
            );
            unsafe {
                (destination as *mut u32).write_unaligned(x.to_bits());
                (destination.wrapping_add(4) as *mut u32).write_unaligned(y.to_bits());
                (destination.wrapping_add(8) as *mut u32).write_unaligned(z.to_bits());
                let _ = lf_checker_rt::callee_thiscall!(5, u32, registry, ped);
            }
        }

        let singleton_slot = lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL);
        let singleton = unsafe { singleton_slot.read_unaligned() };
        if unsafe { read_u8(ped.wrapping_add(PED_ROUTE_STATE)) } != 0 {
            if unsafe { read_u32(ped.wrapping_add(PED_TASK_REGISTRY)) } == 0 {
                return 0;
            }
            let pool = unsafe { lf_checker_rt::callee_thiscall!(6, u32, singleton) };
            if pool == 0 {
                unsafe { write_u8(TASK_ACTIVE, 1) };
            }
            let task = unsafe { lf_checker_rt::callee_thiscall!(7, u32, pool) };
            unsafe { write_u8(task.wrapping_add(TASK_ACTIVE), 1) };
            return task;
        }

        let pool = unsafe { lf_checker_rt::callee_thiscall!(6, u32, singleton) };
        if pool == 0 {
            return 0;
        }
        let route_center = owner.wrapping_add(OWNER_DESTINATION);
        let radius = unsafe { read_u32(owner.wrapping_add(OWNER_RADIUS)) };
        let speed = unsafe { read_u32(owner.wrapping_add(OWNER_SPEED)) };
        unsafe {
            lf_checker_rt::callee_thiscall!(8, u32, pool, radius, route_center, 1, 1, speed)
        }
    }
});
