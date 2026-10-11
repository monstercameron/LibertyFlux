// original: 0x00626D60 network_task_release_owned_members

/// Release the task's optional resource list, its populated embedded member records, and its
/// optional notification object, in the order used by the original method. The task is an
/// x86 object whose first word is replaced with the relocated task vtable. The member list
/// pointer is at +0x166C and is cleared only when both it and its first pointed-to word are
/// nonzero; +0x1678 selects the extra resource-release call. The 32 embedded records begin
/// at +0x958, are spaced by 0x68 bytes, and are visited from highest offset to lowest; a
/// record is released when its word at +8 is nonzero. Finally, a nonzero pointer at +0x18 is
/// passed to the notification-release callee. All engine calls are represented by checker
/// callees so their argument order and effects are observable.
lf_checker_rt::export!(thiscall, rw_00626d60(this: u32) -> () {
    unsafe {
        const TASK_VTABLE: u32 = 0x00FE_4D60;
        const RESOURCE_LIST: u32 = 0x166C;
        const RESOURCE_ENABLED: u32 = 0x1678;
        const NOTIFY_OBJECT: u32 = 0x18;
        const NOTIFY_ARGUMENT: u32 = 0x1670;
        const FIRST_MEMBER: u32 = 0x0958;
        const MEMBER_STRIDE: u32 = 0x68;
        const MEMBER_COUNT: usize = 32;
        const MEMBER_PRESENT: u32 = 0x08;
        const RELEASE_RESOURCE: u32 = 1;
        const RELEASE_LIST: u32 = 2;
        const RELEASE_MEMBER: u32 = 3;
        const RELEASE_NOTIFY: u32 = 4;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let resource_list = read_u32(this.wrapping_add(RESOURCE_LIST));
        if resource_list != 0 && read_u32(resource_list) != 0 {
            if read_u32(this.wrapping_add(RESOURCE_ENABLED)) != 0 {
                write_u32(this.wrapping_add(RESOURCE_LIST), 0);
                let resource_owner = read_u32(resource_list);
                let output_slot = this.wrapping_add(NOTIFY_ARGUMENT);
                let _ = lf_checker_rt::callee_thiscall!(
                    RELEASE_RESOURCE,
                    u32,
                    resource_owner,
                    output_slot
                );
            } else {
                write_u32(this.wrapping_add(RESOURCE_LIST), 0);
            }

            let _ = lf_checker_rt::callee_thiscall!(
                RELEASE_LIST,
                u32,
                resource_list.wrapping_add(4),
                this.wrapping_add(0x1664)
            );
        }

        // The source walks these 32 embedded member records from high to low.
        for member_index in (0..MEMBER_COUNT).rev() {
            
            let member_offset = FIRST_MEMBER
                .wrapping_add((member_index as u32).wrapping_mul(MEMBER_STRIDE));
            let member = this.wrapping_add(member_offset);
            if read_u32(member.wrapping_add(MEMBER_PRESENT)) != 0 {
                let _ = lf_checker_rt::callee_thiscall!(RELEASE_MEMBER, u32, member);
            }
        }

        write_u32(this, lf_checker_rt::relocated(TASK_VTABLE));
        let notify_object = read_u32(this.wrapping_add(NOTIFY_OBJECT));
        if notify_object != 0 {
            let _ = lf_checker_rt::callee_thiscall!(
                RELEASE_NOTIFY,
                u32,
                notify_object,
                this
            );
        }
    }
});

