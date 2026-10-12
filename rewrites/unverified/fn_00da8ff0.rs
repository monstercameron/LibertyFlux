// original: 0x00DA8FF0 task_stuck_in_air_helper

/// Create the first-subtask request for a stuck-in-air task. The first
/// stdcall argument is a signed task-kind switch; the second is the owning task
/// record. Kinds `0xCB`, `0xD0`, and `0xD3` update the request record and ask
/// the task pool for a constructor result. Kind `0x38E` derives a position
/// from the owner's vector record, chooses its direction from the random
/// helper's low bit, and forwards the three exact float words to the
/// constructor. Pool and constructor functions are intercepted callees.
lf_checker_rt::export!(stdcall, rw_00DA8FF0(task_kind: u32, owner: u32) -> u32 {
    unsafe {
        const SCENE_POOL_VA: u32 = 0x0167_E2A0;
        const VECTOR_SCALE_VA: u32 = 0x00FE_8830;
        const OWNER_POSITION: u32 = 0x20;
        const OWNER_SUBTASK_FLAGS: u32 = 0x26C;
        const OWNER_REQUEST: u32 = 0x224;
        const REQUEST_TASK_KIND: u32 = 0x240;
        const REQUEST_STATE: u32 = 0x244;
        const KIND_CB: u32 = 0xCB;
        const KIND_D0: u32 = 0xD0;
        const KIND_D3: u32 = 0xD3;
        const KIND_38E: u32 = 0x38E;
        const KIND_516: u32 = 0x516;
        const POOL_ALLOC: u32 = 1;
        const FALL_AND_GET_UP_CTOR: u32 = 2;
        const JUMP_CTOR: u32 = 3;
        const D3_TASK_CTOR: u32 = 4;
        const DIRECTION_RANDOM: u32 = 5;
        const POSITION_TASK_CTOR: u32 = 6;
        const TASK_CTOR_DURATION: u32 = 0x1388;
        const TASK_CTOR_WEIGHT: u32 = 0x4100_0000;
        const POSITION_TASK_RANGE: u32 = 0x2710;
        const POSITION_TASK_SPEED: u32 = 0x40A0_0000;
        const OWNER_ACTIVE_BIT: u32 = 1;

        #[inline(always)]
        fn multiply(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        #[inline(always)]
        fn add(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        #[inline(always)]
        fn subtract(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn write_u16(address: u32, value: u16) {
            unsafe { (address as *mut u16).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(address) })
        }

        #[inline(always)]
        unsafe fn scene_pool() -> u32 {
            unsafe { lf_checker_rt::global::<u32>(SCENE_POOL_VA).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn allocate_task() -> u32 {
            let pool = unsafe { scene_pool() };
            lf_checker_rt::callee_thiscall!(POOL_ALLOC, u32, pool)
        }

        if (task_kind as i32) > (KIND_D3 as i32) {
            if task_kind != KIND_38E && task_kind != KIND_516 {
                return 0;
            }
            if task_kind == KIND_516 {
                return 0;
            }

            let scale_for_delta = unsafe { read_f32(lf_checker_rt::relocated(VECTOR_SCALE_VA)) };
            let position_for_delta = unsafe { read_u32(owner.wrapping_add(OWNER_POSITION)) };
            unsafe {
                let flags = read_u32(owner.wrapping_add(OWNER_SUBTASK_FLAGS));
                write_u32(owner.wrapping_add(OWNER_SUBTASK_FLAGS), flags | OWNER_ACTIVE_BIT);
            }

            let delta_x = unsafe {
                let scaled = multiply(read_f32(position_for_delta.wrapping_add(0x10)), scale_for_delta);
                subtract(read_f32(position_for_delta.wrapping_add(0x30)), scaled)
            };
            let delta_y = unsafe {
                let scaled = multiply(read_f32(position_for_delta.wrapping_add(0x14)), scale_for_delta);
                subtract(read_f32(position_for_delta.wrapping_add(0x34)), scaled)
            };
            let delta_z = unsafe {
                let scaled = multiply(read_f32(position_for_delta.wrapping_add(0x18)), scale_for_delta);
                subtract(read_f32(position_for_delta.wrapping_add(0x38)), scaled)
            };

            let random_word = lf_checker_rt::callee_cdecl!(DIRECTION_RANDOM, u32);
            let scale_after_random = unsafe { read_f32(lf_checker_rt::relocated(VECTOR_SCALE_VA)) };
            let position_after_random = unsafe { read_u32(owner.wrapping_add(OWNER_POSITION)) };
            let (position_x, position_y, position_z) = if random_word & 1 != 0 {
                unsafe {
                    (
                        add(multiply(read_f32(position_after_random), scale_after_random), delta_x),
                        add(multiply(read_f32(position_after_random + 4), scale_after_random), delta_y),
                        add(multiply(read_f32(position_after_random + 8), scale_after_random), delta_z),
                    )
                }
            } else {
                unsafe {
                    (
                        subtract(delta_x, multiply(read_f32(position_after_random), scale_after_random)),
                        subtract(delta_y, multiply(read_f32(position_after_random + 4), scale_after_random)),
                        subtract(delta_z, multiply(read_f32(position_after_random + 8), scale_after_random)),
                    )
                }
            };

            let allocated = unsafe { allocate_task() };
            if allocated == 0 {
                return 0;
            }
            let position_words = [position_x.to_bits(), position_y.to_bits(), position_z.to_bits()];
            let position_ptr = position_words.as_ptr() as u32;
            return lf_checker_rt::callee_thiscall!(
                POSITION_TASK_CTOR,
                u32,
                allocated,
                position_ptr,
                0,
                POSITION_TASK_SPEED,
                POSITION_TASK_RANGE,
                0
            );
        }

        match task_kind {
            KIND_D0 => {
                let request = unsafe { read_u32(owner.wrapping_add(OWNER_REQUEST)) };
                unsafe {
                    write_u32(request.wrapping_add(REQUEST_TASK_KIND), 0);
                    write_u16(request.wrapping_add(REQUEST_STATE), 0);
                }
                let allocated = unsafe { allocate_task() };
                if allocated == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(
                        FALL_AND_GET_UP_CTOR, u32, allocated, 0x2C, 0xBA, 0x3E8
                    )
                }
            }
            KIND_CB => {
                unsafe {
                    let flags = read_u32(owner.wrapping_add(OWNER_SUBTASK_FLAGS));
                    write_u32(owner.wrapping_add(OWNER_SUBTASK_FLAGS), flags | OWNER_ACTIVE_BIT);
                }
                let allocated = unsafe { allocate_task() };
                if allocated == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(
                        JUMP_CTOR,
                        u32,
                        allocated,
                        TASK_CTOR_DURATION,
                        0,
                        0,
                        TASK_CTOR_WEIGHT
                    )
                }
            }
            KIND_D3 => {
                let request = unsafe { read_u32(owner.wrapping_add(OWNER_REQUEST)) };
                unsafe {
                    let flags = read_u32(owner.wrapping_add(OWNER_SUBTASK_FLAGS));
                    write_u32(owner.wrapping_add(OWNER_SUBTASK_FLAGS), flags | OWNER_ACTIVE_BIT);
                    write_u32(request.wrapping_add(REQUEST_TASK_KIND), 0);
                    write_u16(request.wrapping_add(REQUEST_STATE), 0);
                }
                let allocated = unsafe { allocate_task() };
                if allocated == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(D3_TASK_CTOR, u32, allocated, 1, 0)
                }
            }
            _ => 0,
        }
    }
});
