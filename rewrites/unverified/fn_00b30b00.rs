// original: 0x00B30B00 task_spawn_by_kind

/// Build a task selected by the third cdecl argument. Arguments one and two
/// are the task manager context and a task descriptor. Kinds 1 and 2 use a
/// masked random value to form a signed weight and call the watch-task
/// constructor. Kind 3 checks a returned task's virtual method and may link
/// the descriptor; kind 4 sets bit 2 at byte offset `+0x48` when that method
/// returns zero, otherwise it links the descriptor. Other kinds return zero.
/// Context `+0xb0` must be nonzero and `+0xb8` is compared as signed for the
/// virtual-method path. All constructor and helper calls preserve their
/// original thiscall argument order. The masked random value is at most
/// 65,535; with the executable's two finite scale factors the converted
/// product remains within the signed 32-bit range, so truncation toward zero
/// matches `cvttss2si`.
lf_checker_rt::export!(cdecl, rw_00b30b00(context_ptr: u32, descriptor_ptr: u32, task_kind: u32) -> u32 {
    unsafe {
        const CONTEXT_ACTIVE: u32 = 0xb0;
        const CONTEXT_SIGNED_COUNT: u32 = 0xb8;
        const DESCRIPTOR_KIND: u32 = 0x00;
        const DESCRIPTOR_INPUT: u32 = 0x10;
        const DESCRIPTOR_NESTED: u32 = 0x28;
        const NESTED_DATA: u32 = 0x10;
        const NESTED_CHILD_DATA: u32 = 0x20;
        const CHILD_DATA_BIAS: u32 = 0x30;
        const WATCH_FLAG_BYTE: u32 = 0x48;
        const WATCH_FLAG: u8 = 0x04;
        const WATCH_KIND_FIRST: u32 = 1;
        const WATCH_KIND_LAST: u32 = 2;
        const LINK_KIND_FIRST: u32 = 3;
        const LINK_KIND_LAST: u32 = 4;
        const BASE_WEIGHT: i32 = 2_000;
        const VTABLE_READY_SLOT: u32 = 4;
        const TASK_POOL_MANAGER_VA: u32 = 0x0171faf4;
        const RANDOM_SCALE_VA: u32 = 0x00fe8680;
        const NEGATIVE_WEIGHT_SCALE_VA: u32 = 0x00ead20c;
        const RANDOM_CALLEE: u32 = 1;
        const TASK_POOL_ALLOC_CALLEE: u32 = 2;
        const WATCH_CTOR_CALLEE: u32 = 3;
        const LOOKUP_TASK_CALLEE: u32 = 4;
        const LINK_TASK_CALLEE: u32 = 5;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
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
        fn ordered_mul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        #[inline(always)]
        unsafe fn task_pool_alloc() -> u32 {
            let manager = unsafe {
                lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read()
            };
            lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOC_CALLEE, u32, manager)
        }

        #[inline(always)]
        unsafe fn lookup_task(context_ptr: u32) -> u32 {
            lf_checker_rt::callee_thiscall!(LOOKUP_TASK_CALLEE, u32, context_ptr)
        }

        #[inline(always)]
        unsafe fn task_is_ready(task_ptr: u32) -> u32 {
            let vtable = unsafe { read_u32(task_ptr) };
            let slot = unsafe { read_u32(vtable + VTABLE_READY_SLOT) };
            let method: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(slot as usize) };
            method(task_ptr)
        }

        #[inline(always)]
        unsafe fn link_task(
            allocated_task: u32,
            descriptor_ptr: u32,
        ) -> u32 {
            let nested = unsafe { read_u32(descriptor_ptr + DESCRIPTOR_NESTED) };
            let descriptor_kind = unsafe { read_u32(descriptor_ptr + DESCRIPTOR_KIND) };
            lf_checker_rt::callee_thiscall!(
                LINK_TASK_CALLEE,
                u32,
                allocated_task,
                nested,
                0u32,
                1u32,
                descriptor_kind
            )
        }

        match task_kind {
            WATCH_KIND_FIRST | WATCH_KIND_LAST => {
                let random_value =
                    lf_checker_rt::callee_cdecl!(RANDOM_CALLEE, u32,) & 0xffff;
                let random_fraction = random_value as f32;
                let random_scaled = ordered_mul(
                    random_fraction,
                    unsafe { read_f32(RANDOM_SCALE_VA) },
                );
                let random_weight = ordered_mul(
                    random_scaled,
                    unsafe { read_f32(NEGATIVE_WEIGHT_SCALE_VA) },
                );
                // `random_value <= 65,535`; the original finite factors
                // keep `random_weight` in range for truncating i32 conversion.
                let truncated_weight = random_weight as i32;
                let constructor_weight = BASE_WEIGHT.wrapping_sub(truncated_weight) as u32;
                let nested = unsafe { read_u32(descriptor_ptr + DESCRIPTOR_NESTED) };
                let allocated_task = unsafe { task_pool_alloc() };
                if allocated_task == 0 {
                    return 0;
                }

                let descriptor_kind = unsafe { read_u32(descriptor_ptr + DESCRIPTOR_KIND) };
                if nested != 0 {
                    let child_data = unsafe { read_u32(nested + NESTED_CHILD_DATA) };
                    let constructor_data = if child_data != 0 {
                        child_data.wrapping_add(CHILD_DATA_BIAS)
                    } else {
                        nested.wrapping_add(NESTED_DATA)
                    };
                    lf_checker_rt::callee_thiscall!(
                        WATCH_CTOR_CALLEE,
                        u32,
                        allocated_task,
                        constructor_weight,
                        nested,
                        constructor_data,
                        descriptor_kind
                    )
                } else {
                    lf_checker_rt::callee_thiscall!(
                        WATCH_CTOR_CALLEE,
                        u32,
                        allocated_task,
                        constructor_weight,
                        0u32,
                        descriptor_ptr.wrapping_add(DESCRIPTOR_INPUT),
                        descriptor_kind
                    )
                }
            }
            LINK_KIND_FIRST | LINK_KIND_LAST => {
                let nested = unsafe { read_u32(descriptor_ptr + DESCRIPTOR_NESTED) };
                if nested == 0 {
                    return 0;
                }

                let context_active = unsafe { read_u32(context_ptr + CONTEXT_ACTIVE) };
                let context_count = unsafe { read_u32(context_ptr + CONTEXT_SIGNED_COUNT) } as i32;
                let has_lookup_candidate = context_active != 0 && context_count > 0;
                if has_lookup_candidate {
                    let candidate = unsafe { lookup_task(context_ptr) };
                    if candidate != 0 {
                        let is_ready = unsafe { task_is_ready(candidate) };
                        if task_kind == LINK_KIND_FIRST && is_ready == 0 {
                            return 0;
                        }
                        if task_kind == LINK_KIND_LAST && is_ready == 0 {
                            let flag_address = candidate + WATCH_FLAG_BYTE;
                            let old_flags = unsafe { read_u8(flag_address) };
                            unsafe { write_u8(flag_address, old_flags | WATCH_FLAG) };
                            return 0;
                        }
                    }
                }

                let allocated_task = unsafe { task_pool_alloc() };
                if allocated_task == 0 {
                    return 0;
                }
                unsafe { link_task(allocated_task, descriptor_ptr) }
            }
            _ => 0,
        }
    }
});
