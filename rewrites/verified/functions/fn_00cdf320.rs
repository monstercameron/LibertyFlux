// original: 0x00CDF320 CTaskComplexNM::vf18

/// Update the complex NaturalMotion task. The task's inactive flag is at
/// `+0x24`, its manager pointer at `+0x08`, and the ped behavior pointers are
/// at `+0x38` and `+0x7B4`. The manager helper receives five stack words and
/// the task in ECX; a sign-extended 16-bit field from its result can select an
/// early transition. Otherwise the method calls the manager object's virtual
/// slot at `+0x0C`, then sends the selected action and ped to the task helper.
/// The extracted index is signed; action values are exact unsigned words.
lf_checker_rt::export!(thiscall, rw_00cdf320(this: u32, ped: u32) -> u32 {
    unsafe {
        const INACTIVE: u32 = 0x24;
        const MANAGER: u32 = 0x08;
        const CURRENT_BEHAVIOUR: u32 = 0x38;
        const REQUESTED_BEHAVIOUR: u32 = 0x7B4;
        const MANAGER_VIRTUAL_SLOT: u32 = 0x0C;
        const ACTION_REST: u32 = 0x845;
        const ACTION_FALLBACK: u32 = 0x516;
        const ACTION_MATCHED: u32 = 0x838;
        const MANAGER_ARG_2: u32 = 0x0111_2778;
        const MANAGER_ARG_3: u32 = 0x0111_28A8;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        if unsafe { read_u8(this.wrapping_add(INACTIVE)) } != 0 {
            return 0;
        }

        let manager = unsafe { read_u32(this.wrapping_add(MANAGER)) };
        let relocated_arg_2 = lf_checker_rt::relocated(MANAGER_ARG_2);
        let relocated_arg_3 = lf_checker_rt::relocated(MANAGER_ARG_3);
        let query = lf_checker_rt::callee_thiscall!(
            1, u32, this, manager, 0, relocated_arg_2, relocated_arg_3, 0
        );
        if query != 0 {
            let field = unsafe { read_u32(query.wrapping_add(0x1C)) };
            let signed_index = ((field << 8) as i32) >> 16;
            let current = unsafe { read_u32(ped.wrapping_add(CURRENT_BEHAVIOUR)) };
            let requested = unsafe { read_u32(ped.wrapping_add(REQUESTED_BEHAVIOUR)) };
            if signed_index != -1 {
                if current == 0 || current != requested {
                    return 0;
                }
                return lf_checker_rt::callee_thiscall!(
                    2, u32, this, signed_index as u32, ped
                );
            }
        }

        let manager_vtable = unsafe { read_u32(manager) };
        let method = unsafe { read_u32(manager_vtable.wrapping_add(MANAGER_VIRTUAL_SLOT)) };
        let callback: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(method as usize) };
        let action = callback(manager);
        let current = unsafe { read_u32(ped.wrapping_add(CURRENT_BEHAVIOUR)) };
        let requested = unsafe { read_u32(ped.wrapping_add(REQUESTED_BEHAVIOUR)) };
        let selected_action = if current != 0 && current == requested {
            if action == ACTION_REST { ACTION_MATCHED } else { ACTION_REST }
        } else {
            ACTION_FALLBACK
        };
        lf_checker_rt::callee_thiscall!(2, u32, this, selected_action, ped)
    }
});
