// original: 0x009E5B00 Ped_GiveHelmetItem
/// Update a ped's helmet and prop slots when its active-task and slot checks
/// allow the change. A negative task classification exits through the final
/// status helper. The prop lookup path runs only when the flag bit is clear
/// and either tracked slot is unset. A selector other than `u32::MAX` requests
/// a pose blend. After validation, the helper receives the task class, scaled
/// result, selector, and two temporary output addresses; success sets the
/// update bit and clears the cached value at `+0xE40`.
///
/// All ten direct callees are intercepted. The two pairs of temporary-frame
/// pointers are skipped in call comparison because frame layouts differ; the
/// parent does not read their contents. The helper bodies and their internal
/// object updates are outside this function's proof.
lf_checker_rt::export!(thiscall, rw_009e5b00(
    this: u32,
    selector: u32,
) -> u32 {
    unsafe {
        const PED_FLAGS: u32 = 0x264;
        const SLOT_A: u32 = 0xE48;
        const SLOT_B: u32 = 0xE4C;
        const CACHED_VALUE: u32 = 0xE40;
        const UPDATE_BIT: u32 = 0x8000;
        const NO_SLOT: u32 = u32::MAX;

        let active: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
        if (active as u8) == 0 {
            return lf_checker_rt::callee_thiscall!(10, u32, this);
        }

        let task_class: u32 = lf_checker_rt::callee_thiscall!(2, u32, this);
        if (task_class as i32) < 0 {
            return lf_checker_rt::callee_thiscall!(10, u32, this);
        }

        let flags = (this.wrapping_add(PED_FLAGS) as *const u32).read_unaligned();
        if flags & UPDATE_BIT == 0 {
            let slot_a = (this.wrapping_add(SLOT_A) as *const u32).read_unaligned();
            let slot_b = (this.wrapping_add(SLOT_B) as *const u32).read_unaligned();
            if slot_a == NO_SLOT || slot_b == NO_SLOT {
                let prop_index: u32 = lf_checker_rt::callee_cdecl!(3, u32, this, 0);
                if prop_index != NO_SLOT {
                    let object_slot: u32 = lf_checker_rt::callee_cdecl!(4, u32, this, 0);
                    let _ = lf_checker_rt::callee_thiscall!(
                        5,
                        u32,
                        this,
                        prop_index,
                        object_slot,
                    );
                }
            }
        }

        let mut pose_output_a = core::mem::MaybeUninit::<u32>::uninit();
        let mut pose_output_b = core::mem::MaybeUninit::<u32>::uninit();
        let pose_output_a_ptr = pose_output_a.as_mut_ptr() as u32;
        let pose_output_b_ptr = pose_output_b.as_mut_ptr() as u32;
        if selector != NO_SLOT {
            let _ = lf_checker_rt::callee_thiscall!(
                6,
                u32,
                this,
                pose_output_b_ptr,
                pose_output_a_ptr,
            );
        }

        let scaled_result: u32 = lf_checker_rt::callee_thiscall!(7, u32, this, task_class);
        let validation: u32 = lf_checker_rt::callee_cdecl!(8, u32, this);
        if (validation as u8) >= 2 {
            let prop_index: u32 = lf_checker_rt::callee_cdecl!(3, u32, this, 0);
            if prop_index == NO_SLOT {
                return lf_checker_rt::callee_thiscall!(10, u32, this);
            }
        }

        let _ = lf_checker_rt::callee_cdecl!(
            9,
            u32,
            this,
            0,
            task_class,
            scaled_result,
            selector,
            pose_output_b_ptr,
            pose_output_a_ptr,
        );

        let updated_flags = (this.wrapping_add(PED_FLAGS) as *const u32).read_unaligned()
            | UPDATE_BIT;
        (this.wrapping_add(PED_FLAGS) as *mut u32).write_unaligned(updated_flags);
        (this.wrapping_add(CACHED_VALUE) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(10, u32, this)
    }
});
