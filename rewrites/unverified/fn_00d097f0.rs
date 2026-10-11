// original: 0x00D097F0 CTaskComplexArrestedAIPedAndDriveAway::vf19

/// Build the first task for an arrested AI ped and attach the resulting task
/// event to the ped's intelligence object.
///
/// `owner+0x14` holds the ped context used by the method. A null context
/// returns null without making calls. Otherwise the method allocates and
/// optionally constructs an arrested-and-driven-away task, initializes a
/// stack-local event, asks the ped intelligence at `context+0x224` to attach
/// it, emits the `ARREST_PED` speech using a hash of `ARRESTED`, and marks
/// `owner+0x18` as initialized. It then allocates a follow-up task, stores
/// `5.0f` at its `+0x6c` field, optionally builds a combat-fire subtask, and
/// returns the result of wrapping those tasks in a give-ped-task event.
///
/// Constructors, pool lookups, native helpers, event attachment and event
/// destruction are intercepted by the proof contract. The local event's
/// pointer is stack-relative, so its address is not compared; the contract
/// snapshots its six words at the calls that consume it.
lf_checker_rt::export!(thiscall, rw_00d097f0(owner: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_POOL_GLOBAL: u32 = 0x0167_E2A0;
        const CONTEXT_PED: u32 = 0x14;
        const OWNER_INITIALIZED: u32 = 0x18;
        const CONTEXT_INTELLIGENCE: u32 = 0x224;
        const PED_SPEECH: u32 = 0x570;
        const TASK_CONTEXT: u32 = 0x20;
        const TASK_TIME_LIMIT: u32 = 0x6C;
        const POSITIVE_FIVE_F32: u32 = 0x40A0_0000;
        const ARRESTED_STRING: u32 = 0x00ED_F634;
        const ARREST_PED_STRING: u32 = 0x00ED_F640;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn task_pool() -> u32 {
            let slot = lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL);
            let singleton = unsafe { slot.read_unaligned() };
            unsafe { lf_checker_rt::callee_thiscall!(1, u32, singleton) }
        }

        let context_ped = unsafe { read_u32(owner.wrapping_add(CONTEXT_PED)) };
        if context_ped == 0 {
            return 0;
        }

        let mut local_event = [0u32; 6];
        let local_event_ptr = local_event.as_mut_ptr() as u32;
        let first_pool_object = unsafe { task_pool() };
        let arrested_task = if first_pool_object != 0 {
            unsafe {
                lf_checker_rt::callee_thiscall!(2, u32, first_pool_object, ped)
            }
        } else {
            0
        };

        let _ = unsafe {
            lf_checker_rt::callee_thiscall!(3, u32, local_event_ptr, 2, arrested_task, 0)
        };

        let intelligence = unsafe {
            read_u32(context_ped.wrapping_add(CONTEXT_INTELLIGENCE))
        }
        .wrapping_add(0x84);
        let _ = unsafe {
            lf_checker_rt::callee_thiscall!(4, u32, intelligence, local_event_ptr, 0, 1)
        };

        let speech_hash = unsafe {
            lf_checker_rt::callee_cdecl!(5, u32, ARRESTED_STRING, 0)
        };
        let _ = unsafe {
            lf_checker_rt::callee_thiscall!(
                6,
                u32,
                ped.wrapping_add(PED_SPEECH),
                ARREST_PED_STRING,
                1,
                1,
                0,
                u32::MAX,
                context_ped,
                speech_hash,
                1,
                0,
                0,
            )
        };

        unsafe { write_u32(owner.wrapping_add(OWNER_INITIALIZED), 1) };

        let follow_up_pool_object = unsafe { task_pool() };
        let follow_up_task = if follow_up_pool_object != 0 {
            unsafe {
                lf_checker_rt::callee_thiscall!(
                    7,
                    u32,
                    follow_up_pool_object,
                    context_ped,
                )
            }
        } else {
            0
        };
        unsafe {
            write_u32(
                follow_up_task.wrapping_add(TASK_TIME_LIMIT),
                POSITIVE_FIVE_F32,
            )
        };

        let give_task_pool_object = unsafe { task_pool() };
        let give_task = if give_task_pool_object != 0 {
            let combat_task_pool_object = unsafe { task_pool() };
            let combat_task = if combat_task_pool_object != 0 {
                let task_context = unsafe {
                    read_u32(context_ped.wrapping_add(TASK_CONTEXT))
                }
                .wrapping_add(0x30);
                unsafe {
                    lf_checker_rt::callee_thiscall!(
                        8,
                        u32,
                        combat_task_pool_object,
                        context_ped,
                        task_context,
                        1,
                        1,
                        0,
                    )
                }
            } else {
                0
            };

            unsafe {
                lf_checker_rt::callee_thiscall!(
                    9,
                    u32,
                    give_task_pool_object,
                    local_event_ptr,
                    combat_task,
                    0,
                    0,
                )
            }
        } else {
            0
        };

        let _ = unsafe { lf_checker_rt::callee_thiscall!(10, u32, local_event_ptr) };
        give_task
    }
});
