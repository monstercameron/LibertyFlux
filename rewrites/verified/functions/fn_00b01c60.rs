// original: 0x00B01C60 update_view_and_children

/// Refreshes the view when its saved version differs from the current one,
/// updates the root parameter buffer, and visits each child. A child is
/// dispatched only when its vtable predicate returns nonzero. If the saved
/// version already matches, the function returns that version immediately;
/// otherwise it returns the child count after the refresh sequence.
lf_checker_rt::export!(thiscall, rw_00b01c60(this: u32) -> u32 {
    unsafe {
        const CURRENT_VERSION: u32 = 0x018B6C84;
        const CURRENT_ROOT: u32 = 0x018B6EF0;
        const SAVED_VERSION: u32 = 0x53C;
        const CHILDREN_POINTER: u32 = 0x400;
        const CHILD_COUNT: u32 = 0x404;
        const PARAMETER: u32 = 0x40C;
        const FIRST_ARGUMENT: u32 = 0x548;
        const SECOND_ARGUMENT: u32 = 0x550;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            (address as *const u32).read_unaligned()
        }

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            (address as *const u16).read_unaligned()
        }

        let saved_version = read_u32(this.wrapping_add(SAVED_VERSION));
        let current_version = lf_checker_rt::global::<u32>(CURRENT_VERSION).read();
        if saved_version == current_version {
            return saved_version;
        }

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this.wrapping_add(0x10));
        let current_root = lf_checker_rt::global::<u32>(CURRENT_ROOT).read();
        if current_root != this {
            let _ = lf_checker_rt::callee_thiscall!(2, u32, this);
        }

        let mut parameters = [0u32; 3];
        let _ = lf_checker_rt::callee_thiscall!(3, u32, parameters.as_mut_ptr() as u32);
        let _ = lf_checker_rt::callee_thiscall!(3, u32, parameters.as_mut_ptr() as u32);
        parameters[0] = read_u32(this.wrapping_add(PARAMETER));
        let _ = lf_checker_rt::callee_thiscall!(4, u32, this);

        let root_vtable = read_u32(this);
        let root_dispatch = read_u32(root_vtable.wrapping_add(0x30));
        let root_method: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(root_dispatch as usize);
        let _ = root_method(this, parameters.as_mut_ptr() as u32);

        let children = read_u32(this.wrapping_add(CHILDREN_POINTER));
        let count = read_u16(this.wrapping_add(CHILD_COUNT)) as u32;
        let mut index = 0u32;
        while index < count {
            let child = read_u32(children.wrapping_add(index.wrapping_mul(4)));
            let child_vtable = read_u32(child);
            let predicate_address = read_u32(child_vtable.wrapping_add(0x34));
            let predicate: extern "thiscall" fn(u32) -> u8 =
                core::mem::transmute(predicate_address as usize);
            if predicate(child) != 0 {
                let action_address = read_u32(child_vtable.wrapping_add(0x2C));
                let action: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(action_address as usize);
                let result = action(
                    child,
                    read_u32(this.wrapping_add(SECOND_ARGUMENT)),
                    read_u32(this.wrapping_add(FIRST_ARGUMENT)),
                );
                let _ = lf_checker_rt::callee_thiscall!(8, u32, result);
            }
            index = index.wrapping_add(1);
        }

        count
    }
});
