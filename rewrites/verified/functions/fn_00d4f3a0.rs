// original: 0x00D4F3A0 CTaskComplexPickUpObject::vf19

/// Chooses the next pickup subtask for a complex pickup task.
///
/// `this` is the complex task, whose current child pointer is at `+0x14`.
/// `target_ped` is the pedestrian argument. The child task's kind byte at
/// `+0x1e2` gates an early object-identity check against its pointer at
/// `+0x1bc`; otherwise a virtual child method fills a three-float position.
/// The function compares that position with the target's transform position
/// (pointer at `+0x20`, vector at `+0x30`) and applies the original scalar
/// distance calculation and threshold. Distant positions delegate task
/// creation to the far-position helper. Near positions use the shared pickup
/// context and choose one of two task factories. Pedestrian states 3 and 4
/// take a separate factory path and initialise the returned task fields.
///
/// The virtual output buffer is local to this call. Its address is therefore
/// not compared; the contract compares its three words at the call site.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00d4f3a0(this: u32, target_ped: u32) -> u32 {
    unsafe {
        const CHILD_TASK: u32 = 0x14;
        const CHILD_OBJECT: u32 = 0x1bc;
        const CHILD_KIND: u32 = 0x1e2;
        const PED_STATE: u32 = 0xb80;
        const PED_TRANSFORM: u32 = 0x20;
        const TRANSFORM_POSITION: u32 = 0x30;
        const DISTANCE_THRESHOLD: u32 = 0x00fe_87e4;
        const STEP_SCALE: u32 = 0x00fe_8810;
        const PICKUP_CONTEXT: u32 = 0x0167_e2a0;
        const NEW_TASK_VTABLE_VA: u32 = 0x00eb_391c;

        const GET_CHILD_POSITION: u32 = 1;
        const SQUARE_ROOT: u32 = 2;
        const CREATE_FAR_SUBTASK: u32 = 3;
        const CHECK_NEAR_TARGET: u32 = 4;
        const GET_NEAR_CONTEXT: u32 = 5;
        const MAKE_NEAR_SUBTASK: u32 = 6;
        const GET_FALLBACK_CONTEXT: u32 = 7;
        const GET_STATE_CONTEXT: u32 = 8;
        const MAKE_FALLBACK_SUBTASK: u32 = 9;
        const INITIALISE_NEW_TASK: u32 = 10;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
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
        unsafe fn global_f32(file_va: u32) -> f32 {
            unsafe { lf_checker_rt::global::<f32>(file_va).read_unaligned() }
        }
        #[inline(always)]
        fn add(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }
        #[inline(always)]
        fn sub(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }
        #[inline(always)]
        fn mul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        let child = read_u32(this + CHILD_TASK);
        if child == 0 {
            return 0;
        }

        let child_kind = read_u8(child + CHILD_KIND) & 0x0f;
        if child_kind >= 2 && read_u32(child + CHILD_OBJECT) == target_ped {
            return 0;
        }

        let mut child_position_bits = [0u32; 3];
        let child_position_ptr = child_position_bits.as_mut_ptr() as u32;
        let _ = lf_checker_rt::callee_thiscall!(
            GET_CHILD_POSITION,
            u32,
            child,
            child_position_ptr
        );
        let child_x = f32::from_bits(child_position_bits[0]);
        let child_y = f32::from_bits(child_position_bits[1]);
        let child_z = f32::from_bits(child_position_bits[2]);

        let pedestrian_state = read_u32(target_ped + PED_STATE);
        if pedestrian_state == 3 || pedestrian_state == 4 {
            let context = lf_checker_rt::global::<u32>(PICKUP_CONTEXT).read_unaligned();
            let task = lf_checker_rt::callee_thiscall!(GET_STATE_CONTEXT, u32, context);
            if task == 0 {
                return 0;
            }
            let _ = lf_checker_rt::callee_thiscall!(INITIALISE_NEW_TASK, u32, task);
            let relocated_task_vtable = lf_checker_rt::global::<u8>(NEW_TASK_VTABLE_VA) as u32;
            write_u32(task, relocated_task_vtable);
            write_u32(task + 0x14, 0);
            write_u8(task + 0x18, 0);
            write_u32(task + 0x1c, 0);
            return task;
        }

        let pedestrian_transform = read_u32(target_ped + PED_TRANSFORM);
        let pedestrian_x = read_f32(pedestrian_transform + TRANSFORM_POSITION);
        let pedestrian_y = read_f32(pedestrian_transform + TRANSFORM_POSITION + 4);
        let pedestrian_z = read_f32(pedestrian_transform + TRANSFORM_POSITION + 8);
        let delta_x = sub(pedestrian_x, child_x);
        let delta_y = sub(pedestrian_y, child_y);
        let delta_z = sub(pedestrian_z, child_z);

        // The original accumulates Y squared, then X squared, then Z squared.
        let distance_squared = add(
            add(mul(delta_y, delta_y), mul(delta_x, delta_x)),
            mul(delta_z, delta_z),
        );
        let distance = lf_checker_rt::callee_cdecl!(
            SQUARE_ROOT,
            f32,
            distance_squared.to_bits()
        );
        let step_scale = global_f32(STEP_SCALE);
        let advanced_y = add(mul(mul(delta_z, distance), step_scale), child_y);
        let advanced_z = add(mul(mul(delta_y, distance), step_scale), child_z);

        let remaining_x = sub(pedestrian_x, advanced_y);
        let remaining_y = sub(pedestrian_y, advanced_z);
        let remaining_squared = add(
            mul(remaining_y, remaining_y),
            mul(remaining_x, remaining_x),
        );
        let threshold = global_f32(DISTANCE_THRESHOLD);
        // COMISS followed by JBE takes the near branch for unordered values.
        if remaining_squared > threshold {
            return lf_checker_rt::callee_cdecl!(
                CREATE_FAR_SUBTASK,
                u32,
                child_position_ptr
            );
        }

        let use_near_factory = lf_checker_rt::callee_cdecl!(
            CHECK_NEAR_TARGET,
            u8,
            this + 0x3c
        ) != 0;
        let context = lf_checker_rt::global::<u32>(PICKUP_CONTEXT).read_unaligned();
        if use_near_factory {
            let context = lf_checker_rt::callee_thiscall!(GET_NEAR_CONTEXT, u32, context);
            if context == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(
                MAKE_NEAR_SUBTASK,
                u32,
                context,
                read_u32(this + CHILD_TASK),
                read_u32(this + 0x18)
            );
        }

        let context = lf_checker_rt::callee_thiscall!(GET_FALLBACK_CONTEXT, u32, context);
        if context == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(MAKE_FALLBACK_SUBTASK, u32, context, 0, 0)
    }
}
