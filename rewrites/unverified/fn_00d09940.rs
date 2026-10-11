// original: 0x00D09940 CTaskComplexBackOff::vf19

/// Select the first task for the ped managed by a back-off task.
///
/// The method first checks the incoming ped's state at `+0x21c` and `+0x26c`,
/// then reads the task context from `self+0x14`. For an eligible context it
/// checks the context flags at `+0x28`, state bytes at `+0x218`, `+0x219`, and
/// `+0x211`, and a state word at `+0xa70`. The helper at `context+0x224` may
/// return an object whose vtable slot 3 supplies the task type; type `0x454`
/// selects the combat-fire constructor. A random value is compared as signed
/// against `0x3fff` when the context state is not `1`.
///
/// Eligible contexts create a combat-fire task. Other valid peds take the
/// follow-nav-mesh route constructor, while failed top-level checks create a
/// simple task and initialize its vtable and state fields. Pool, type-query,
/// random, and constructor calls are intercepted by the contract. The original
/// reuses its incoming ped argument slot for the random-selection byte; that
/// byte is observed in the combat-fire constructor call arguments.
lf_checker_rt::export!(thiscall, rw_00d09940(owner: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_POOL_GLOBAL: u32 = 0x0167_E2A0;
        const SIMPLE_TASK_VTABLE: u32 = 0x00EB_391C;
        const ROUTE_FLOAT: u32 = 0x00EE_1EB4;
        const PED_STATE: u32 = 0x21C;
        const PED_FLAGS: u32 = 0x26C;
        const OWNER_CONTEXT: u32 = 0x14;
        const CONTEXT_FLAGS: u32 = 0x28;
        const CONTEXT_NESTED: u32 = 0x20;
        const CONTEXT_STATE_A: u32 = 0x218;
        const CONTEXT_STATE_B: u32 = 0x219;
        const CONTEXT_STATE_C: u32 = 0x211;
        const CONTEXT_MODE: u32 = 0xA70;
        const CONTEXT_QUERY: u32 = 0x224;
        const STATE_KIND: u32 = 0x12C;
        const PED_STATE_KIND: u32 = 2;
        const PED_BLOCK_MASK: u8 = 4;
        const CONTEXT_FLAG_MASK: u32 = 0x3C0;
        const CONTEXT_FLAGS_REQUIRED: u32 = 0xC0;
        const QUERY_TASK_TYPE: u32 = 0x454;
        const RANDOM_LIMIT: i32 = 0x3FFF;
        const STACK_ARG_WITHOUT_LOW_BYTE: u32 = 0xFFFF_FF00;
        const ROUTE_MODE: u32 = 2;
        const ROUTE_TIME: u32 = 0x42C8_0000;

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
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn task_pool() -> u32 {
            let singleton_slot = lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL);
            let singleton = unsafe { singleton_slot.read_unaligned() };
            unsafe { lf_checker_rt::callee_thiscall!(5, u32, singleton) }
        }

        #[inline(always)]
        unsafe fn nested_route(context: u32) -> u32 {
            let nested = unsafe { read_u32(context.wrapping_add(CONTEXT_NESTED)) };
            if nested != 0 {
                nested.wrapping_add(0x30)
            } else {
                context.wrapping_add(0x10)
            }
        }

        let state = unsafe { read_u32(ped.wrapping_add(PED_STATE)) };
        let blocked_by_state = unsafe {
            read_u32(state.wrapping_add(STATE_KIND)) != PED_STATE_KIND
        };
        let blocked_by_flag = unsafe { read_u8(ped.wrapping_add(PED_FLAGS)) & PED_BLOCK_MASK } != 0;
        let context = unsafe { read_u32(owner.wrapping_add(OWNER_CONTEXT)) };

        if blocked_by_state || blocked_by_flag || context == 0 {
            let task = unsafe { task_pool() };
            if task == 0 {
                return 0;
            }
            let _ = unsafe { lf_checker_rt::callee_thiscall!(9, u32, task) };
            let vtable = lf_checker_rt::global::<u8>(SIMPLE_TASK_VTABLE) as u32;
            unsafe {
                write_u32(task, vtable);
                write_u32(task.wrapping_add(0x14), 0);
                write_u8(task.wrapping_add(0x18), 0);
                write_u32(task.wrapping_add(0x1C), 0);
            }
            return task;
        }

        let flags = unsafe { read_u32(context.wrapping_add(CONTEXT_FLAGS)) };
        let eligible_context = if flags & CONTEXT_FLAG_MASK == CONTEXT_FLAGS_REQUIRED {
            context
        } else {
            0
        };

        let mut create_combat_task = false;
        if eligible_context != 0
            && unsafe { read_u8(context.wrapping_add(CONTEXT_STATE_A)) } == 0
            && unsafe { read_u8(context.wrapping_add(CONTEXT_STATE_B)) } != 0
        {
            if unsafe { read_u32(context.wrapping_add(CONTEXT_MODE)) } == 1 {
                create_combat_task = true;
            } else {
                let query_this = unsafe { read_u32(context.wrapping_add(CONTEXT_QUERY)) };
                let first_query = unsafe {
                    lf_checker_rt::callee_thiscall!(1, u32, query_this)
                };
                if first_query != 0 {
                    let query_object = unsafe {
                        lf_checker_rt::callee_thiscall!(1, u32, query_this)
                    };
                    let vtable = unsafe { read_u32(query_object) };
                    let method = unsafe { read_u32(vtable.wrapping_add(0x0C)) };
                    let get_task_type: extern "thiscall" fn(u32) -> u32 =
                        unsafe { core::mem::transmute(method as usize) };
                    create_combat_task = unsafe { get_task_type(query_object) } == QUERY_TASK_TYPE;
                }
            }
        }

        if eligible_context != 0 && !create_combat_task {
            let has_first_task = unsafe {
                lf_checker_rt::callee_thiscall!(3, u32, eligible_context)
            } & 0xFF
                != 0;
            if has_first_task
                && unsafe { read_u8(eligible_context.wrapping_add(CONTEXT_STATE_C)) } != 0
            {
                create_combat_task = true;
            }
        }

        if eligible_context != 0 && create_combat_task {
            let random_selection = if unsafe {
                read_u32(eligible_context.wrapping_add(CONTEXT_MODE))
            } == 1
            {
                0
            } else {
                let random_value = unsafe { lf_checker_rt::callee_cdecl!(4, u32,) };
                ((random_value as i32) < RANDOM_LIMIT) as u32
            };
            let randomized_ped_argument =
                (ped & STACK_ARG_WITHOUT_LOW_BYTE) | random_selection;
            let task = unsafe { task_pool() };
            if task == 0 {
                return 0;
            }
            let route = unsafe { nested_route(context) };
            return unsafe {
                lf_checker_rt::callee_thiscall!(
                    6,
                    u32,
                    task,
                    context,
                    route,
                    randomized_ped_argument,
                    1,
                    0,
                )
            };
        }

        let route_task = unsafe { task_pool() };
        if route_task == 0 {
            return 0;
        }
        let follow_task_pool = unsafe { task_pool() };
        let follow_task = if follow_task_pool != 0 {
            let route = unsafe { nested_route(context) };
            let route_float_slot = lf_checker_rt::global::<u32>(ROUTE_FLOAT);
            let route_float = unsafe { route_float_slot.read_unaligned() };
            unsafe {
                lf_checker_rt::callee_thiscall!(
                    7,
                    u32,
                    follow_task_pool,
                    ROUTE_MODE,
                    route,
                    ROUTE_TIME,
                    route_float,
                    u32::MAX,
                    1,
                    1,
                    0,
                    0,
                    1,
                )
            }
        } else {
            0
        };
        unsafe {
            lf_checker_rt::callee_thiscall!(8, u32, route_task, follow_task, 0, 0)
        }
    }
});
